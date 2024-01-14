use std::mem;
use std::sync::mpsc::sync_channel;
use std::sync::Arc;

use crate::renderers::renderer::Renderer;
use anyhow::Result;
use cosmic_text::{SubpixelBin, SwashCache, TextLayout};
use crate::*;
use image::{DynamicImage, EncodableLayout, RgbaImage};
use vger::{Image, PaintIndex, PixelFormat, Vger};
use wgpu::{Device, DeviceType, Queue, StoreOp, Surface, SurfaceConfiguration, TextureFormat};

pub struct VgerRenderer {
    device: Arc<Device>,
    #[allow(unused)]
    queue: Arc<Queue>,
    surface: Surface,
    vger: Vger,
    alt_vger: Option<Vger>,
    pub config: SurfaceConfiguration,
    scale: f32,
    transform: LocalToWorld,
    clip: Option<LocalRect>,
    capture: bool,
}

const CLEAR_COLOR: wgpu::Color = wgpu::Color {
    r: 0.0,
    g: 0.0,
    b: 0.0,
    a: 0.0,
};

impl VgerRenderer {
    pub fn new<
        W: raw_window_handle::HasRawDisplayHandle + raw_window_handle::HasRawWindowHandle,
    >(
        window: &W,
        width: u32,
        height: u32,
        scale: f32,
    ) -> Result<Self> {
        let instance = wgpu::Instance::default();

        let surface = unsafe { instance.create_surface(window) }?;

        let adapter =
            futures::executor::block_on(instance.request_adapter(&wgpu::RequestAdapterOptions {
                power_preference: wgpu::PowerPreference::default(),
                compatible_surface: Some(&surface),
                force_fallback_adapter: false,
            }))
            .ok_or_else(|| anyhow::anyhow!("can't get adapter"))?;

        if adapter.get_info().device_type == DeviceType::Cpu {
            return Err(anyhow::anyhow!("only cpu adapter found"));
        }

        let mut required_downlevel_flags = wgpu::DownlevelFlags::empty();
        required_downlevel_flags.set(wgpu::DownlevelFlags::VERTEX_STORAGE, true);

        if !adapter
            .get_downlevel_capabilities()
            .flags
            .contains(required_downlevel_flags)
        {
            return Err(anyhow::anyhow!(
                "adapter doesn't support required downlevel flags"
            ));
        }

        let (device, queue) = futures::executor::block_on(adapter.request_device(
            &wgpu::DeviceDescriptor {
                features: wgpu::Features::empty(),
                limits: wgpu::Limits::default(),
                label: None,
            },
            None,
        ))?;
        let device = Arc::new(device);
        let queue = Arc::new(queue);

        let surface_caps = surface.get_capabilities(&adapter);
        let texture_format = surface_caps
            .formats
            .into_iter()
            .find(|it| matches!(it, TextureFormat::Rgba8Unorm | TextureFormat::Bgra8Unorm))
            .ok_or_else(|| anyhow::anyhow!("surface should support Rgba8Unorm or Bgra8Unorm"))?;

        let config = wgpu::SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format: texture_format,
            width,
            height,
            present_mode: wgpu::PresentMode::Fifo,
            alpha_mode: wgpu::CompositeAlphaMode::Auto,
            view_formats: vec![],
        };
        surface.configure(&device, &config);

        let vger = vger::Vger::new(device.clone(), queue.clone(), texture_format);

        Ok(Self {
            device,
            queue,
            surface,
            vger,
            alt_vger: None,
            scale,
            config,
            transform: LocalToWorld::identity(),
            clip: None,
            capture: false,
        })
    }

    pub fn resize(&mut self, width: u32, height: u32, scale: f32) {
        if width != self.config.width || height != self.config.height {
            self.config.width = width;
            self.config.height = height;
            self.surface.configure(&self.device, &self.config);
        }
        self.scale = scale;
    }

    pub fn set_scale(&mut self, scale: f32) {
        self.scale = scale;
    }
}

impl VgerRenderer {
    fn brush_to_paint<'b>(&mut self, brush: Paint) -> Option<PaintIndex> {
        let paint = match brush {
            Paint::Color(color) => self.vger.color_paint(color),
            Paint::Gradient {
                start,
                end,
                inner_color,
                outer_color,
            } => self.vger.linear_gradient(start, end, inner_color, outer_color, 0.0),
        };
        Some(paint)
    }

    fn vger_point(&self, point: LocalPoint) -> vger::defs::LocalPoint {
        let point = point + LocalOffset::new(self.transform.m31, self.transform.m32);
        vger::defs::LocalPoint::new(
            (point.x * self.scale).round() as f32,
            (point.y * self.scale).round() as f32,
        )
    }

    fn vger_rect(&self, rect: LocalRect) -> vger::defs::LocalRect {
        let origin = rect.origin;
        let origin = self.vger_point(origin);

        let end = LocalPoint::new(rect.min_x(), rect.min_y());
        let end = self.vger_point(end);

        let size = (end - origin).to_size();
        vger::defs::LocalRect::new(origin, size)
    }

    fn render_image(&mut self) -> Option<DynamicImage> {
        let width_align = wgpu::COPY_BYTES_PER_ROW_ALIGNMENT - 1;
        let width = (self.config.width + width_align) & !width_align;
        let height = self.config.height;
        let texture_desc = wgpu::TextureDescriptor {
            size: wgpu::Extent3d {
                width: self.config.width,
                height,
                depth_or_array_layers: 1,
            },
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format: wgpu::TextureFormat::Rgba8Unorm,
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT | wgpu::TextureUsages::COPY_SRC,
            label: Some("render_texture"),
            view_formats: &[wgpu::TextureFormat::Rgba8Unorm],
        };
        let texture = self.device.create_texture(&texture_desc);
        let view = texture.create_view(&wgpu::TextureViewDescriptor::default());
        let desc = wgpu::RenderPassDescriptor {
            color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                view: &view,
                resolve_target: None,
                ops: wgpu::Operations {
                    load: wgpu::LoadOp::Clear(CLEAR_COLOR),
                    store: StoreOp::Store,
                },
            })],
            ..Default::default()
        };

        self.vger.encode(&desc);

        let bytes_per_pixel = 4;
        let buffer = self.device.create_buffer(&wgpu::BufferDescriptor {
            label: None,
            size: (width as u64 * height as u64 * bytes_per_pixel),
            usage: wgpu::BufferUsages::MAP_READ | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });
        let bytes_per_row = width * bytes_per_pixel as u32;
        assert!(bytes_per_row % wgpu::COPY_BYTES_PER_ROW_ALIGNMENT == 0);

        let mut encoder = self
            .device
            .create_command_encoder(&wgpu::CommandEncoderDescriptor { label: None });
        encoder.copy_texture_to_buffer(
            texture.as_image_copy(),
            wgpu::ImageCopyBuffer {
                buffer: &buffer,
                layout: wgpu::ImageDataLayout {
                    offset: 0,
                    bytes_per_row: Some(bytes_per_row),
                    rows_per_image: None,
                },
            },
            texture_desc.size,
        );
        let command_buffer = encoder.finish();
        self.queue.submit(Some(command_buffer));
        self.device.poll(wgpu::Maintain::Wait);

        let slice = buffer.slice(..);
        let (tx, rx) = sync_channel(1);
        slice.map_async(wgpu::MapMode::Read, move |r| tx.send(r).unwrap());

        loop {
            if let Ok(r) = rx.try_recv() {
                break r.ok()?;
            }
            if self.device.poll(wgpu::MaintainBase::Wait) {
                rx.recv().ok()?.ok()?;
                break;
            }
        }

        let mut cropped_buffer = Vec::new();
        let buffer: Vec<u8> = slice.get_mapped_range().to_owned();

        let mut cursor = 0;
        let row_size = self.config.width as usize * bytes_per_pixel as usize;
        for _ in 0..height {
            cropped_buffer.extend_from_slice(&buffer[cursor..(cursor + row_size)]);
            cursor += bytes_per_row as usize;
        }

        RgbaImage::from_raw(self.config.width, height, cropped_buffer).map(DynamicImage::ImageRgba8)
    }
}

impl Renderer for VgerRenderer {
    fn begin(&mut self, capture: bool) {
        // Switch to the capture Vger if needed
        if self.capture != capture {
            self.capture = capture;
            if self.alt_vger.is_none() {
                self.alt_vger = Some(vger::Vger::new(
                    self.device.clone(),
                    self.queue.clone(),
                    TextureFormat::Rgba8Unorm,
                ));
            }
            mem::swap(&mut self.vger, self.alt_vger.as_mut().unwrap())
        }

        self.transform = LocalToWorld::identity();
        self.vger.begin(
            self.config.width as f32,
            self.config.height as f32,
            self.scale as f32,
        );
    }

    fn stroke(&mut self, shape: Shape, brush: Paint, width: f32) {
        let paint = match self.brush_to_paint(brush) {
            Some(paint) => paint,
            None => return,
        };

        let width = (width * self.scale).round() as f32;
        match shape {
            Shape::Rectangle(rect, corner_radius) => {
                self.vger.stroke_rect(
                    rect.min(),
                    rect.max(),
                    corner_radius,
                    width,
                    paint,
                );
            },
            Shape::Circle(center, radius) => {},
        }
        // } else if let Some(line) = shape.as_line() {
        //     self.vger.stroke_segment(
        //         self.vger_point(line.p0),
        //         self.vger_point(line.p1),
        //         width,
        //         paint,
        //     );
        // }
    }

    fn fill(&mut self, path: Shape, brush: Paint, blur_radius: f32) {
        let paint = match self.brush_to_paint(brush) {
            Some(paint) => paint,
            None => return,
        };

        match path {
            Shape::Rectangle(rect, corner_radius) => {
                self.vger.fill_rect(
                    *rect,
                    corner_radius,
                    paint,
                    blur_radius * self.scale,
                );
            },
            Shape::Circle(center, radius) => {
                self.vger.fill_circle(*center, radius, paint)
            },
            // None => self.vger.fill(paint)
         }
        // } else if let Some(rect) = path.as_rounded_rect() {
        //     self.vger.fill_rect(
        //         self.vger_rect(rect.rect()),
        //         (rect.radii().top_left * self.scale) as f32,
        //         paint,
        //         (blur_radius * self.scale) as f32,
        //     );
    }

    fn draw_text(&mut self, layout: &TextLayout, pos: LocalPoint) {
        let mut swash_cache = SwashCache::new();
        let offset = LocalOffset::new(self.transform.m31, self.transform.m32);
        // let pos: LocalPoint = pos.into();
        let clip = self.clip;
        for line in layout.layout_runs() {
            if let Some(rect) = clip {
                let y = pos.y + offset.y + line.line_y;
                if y + line.line_height < rect.min_y() {
                    continue;
                }
                if y - line.line_height > rect.max_y() {
                    break;
                }
            }
            'line_loop: for glyph_run in line.glyphs {
                let x = glyph_run.x + pos.x as f32 + offset.x as f32;
                let y = line.line_y + pos.y as f32 + offset.y as f32;

                if let Some(rect) = clip {
                    if (x + glyph_run.w) < rect.min_x() {
                        continue;
                    } else if x > rect.max_x() {
                        break 'line_loop;
                    }
                }

                // if glyph_run.is_tab {
                //     continue;
                // }

                if let Some(paint) = self.brush_to_paint(Paint::Color(glyph_run.color)) {
                    let glyph_x = x * self.scale as f32;
                    let (new_x, subpx_x) = SubpixelBin::new(glyph_x);
                    let glyph_x = new_x as f32;

                    let glyph_y = (y * self.scale as f32).round();
                    let (new_y, subpx_y) = SubpixelBin::new(glyph_y);
                    let glyph_y = new_y as f32;

                    let font_size = (glyph_run.font_size * self.scale as f32).round() as u32;
                    self.vger.render_glyph(
                        glyph_x,
                        glyph_y,
                        glyph_run.cache_key.font_id,
                        glyph_run.cache_key.glyph_id,
                        font_size,
                        (subpx_x, subpx_y),
                        || {
                            let mut cache_key = glyph_run.cache_key;
                            cache_key.font_size = font_size;
                            cache_key.x_bin = subpx_x;
                            cache_key.y_bin = subpx_y;
                            let image = swash_cache.get_image_uncached(cache_key);
                            image.unwrap_or_default()
                        },
                        paint,
                    );
                }
            }
        }
    }

    // fn draw_img(&mut self, img: Img<'_>, rect: LocalRect) {
    //     let transform = self.transform.as_coeffs();
    //     let width = (rect.width() * self.scale).round() as u32;
    //     let height = (rect.height() * self.scale).round() as u32;
    //     let width = width.max(1);
    //     let height = height.max(1);
    //     let origin = rect.origin();
    //     let x = ((origin.x + transform[4]) * self.scale).round() as f32;
    //     let y = ((origin.y + transform[5]) * self.scale).round() as f32;

    //     self.vger.render_image(x, y, img.hash, width, height, || {
    //         let rgba = img.img.clone().into_rgba8();
    //         let data = rgba.as_bytes().to_vec();

    //         let (width, height) = rgba.dimensions();
    //         Image {
    //             width,
    //             height,
    //             data,
    //             pixel_format: PixelFormat::Rgba,
    //         }
    //     });
    // }

    fn transform(&mut self, transform: LocalToWorld) {
        self.transform = transform;
    }

    fn set_z_index(&mut self, z_index: i32) {
        self.vger.set_z_index(z_index);
    }

    fn clip(&mut self, shape: Shape) {
        let (rect, radius) = if let Some(rect) = shape.as_rect() {
            (rect, 0.0)
        } else if let Some(rect) = shape.as_rounded_rect() {
            (rect.rect(), rect.radii().top_left)
        } else {
            (shape.bounding_box(), 0.0)
        };

        self.vger
            .scissor(self.vger_rect(rect), (radius * self.scale) as f32);

        let transform = self.transform.as_coeffs();
        let offset = LocalOffset::new(transform[4], transform[5]);
        self.clip = Some(rect + offset);
    }

    fn clear_clip(&mut self) {
        self.vger.reset_scissor();
        self.clip = None;
    }

    fn finish(&mut self) -> Option<DynamicImage> {
        if self.capture {
            self.render_image()
        } else {
            if let Ok(frame) = self.surface.get_current_texture() {
                let texture_view = frame
                    .texture
                    .create_view(&wgpu::TextureViewDescriptor::default());
                let desc = wgpu::RenderPassDescriptor {
                    label: None,
                    color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                        view: &texture_view,
                        resolve_target: None,
                        ops: wgpu::Operations {
                            load: wgpu::LoadOp::Clear(CLEAR_COLOR),
                            store: StoreOp::Store,
                        },
                    })],
                    depth_stencil_attachment: None,
                    timestamp_writes: None,
                    occlusion_query_set: None,
                };

                self.vger.encode(&desc);
                frame.present();
            }
            None
        }
    }
}

fn vger_color(color: Color) -> vger::Color {
    vger::Color {
        r: color.r as f32 / 255.0,
        g: color.g as f32 / 255.0,
        b: color.b as f32 / 255.0,
        a: color.a as f32 / 255.0,
    }
}
