use cosmic_text::TextLayout;
use image::DynamicImage;

use crate::*;

pub struct Img<'a> {
    pub img: &'a DynamicImage,
    pub data: &'a [u8],
    pub hash: &'a [u8],
}

pub trait Renderer {
    fn begin(&mut self, capture: bool);

    fn restore(&mut self) {}

    fn translate(&mut self, offiset: LocalOffset, save: bool) {}

    fn transform(&mut self, transform: WorldToLocal);

    fn current_tranform(&self) -> LocalToWorld;

    fn set_z_index(&mut self, z_index: i32);

    /// Clip to a [`Shape`].
    fn clip(&mut self, shape: Shape);

    fn clear_clip(&mut self);

    /// Stroke a [`Shape`].
    fn stroke(&mut self, shape: Shape, brush: Paint, width: f32);

    /// Fill a [`Shape`], using the [non-zero fill rule].
    ///
    /// [non-zero fill rule]: https://en.wikipedia.org/wiki/Nonzero-rule
    fn fill(&mut self, path: Shape, brush: Paint, blur_radius: f32);

    /// Draw a [`TextLayout`].
    ///
    /// The `pos` parameter specifies the upper-left corner of the layout object
    /// (even for right-to-left text).
    fn draw_text(&mut self, layout: &TextLayout, pos: LocalPoint);

    // fn draw_img(&mut self, img: Img<'_>, rect: LocalRect);

    fn finish(&mut self) -> Option<DynamicImage>;
}

pub struct EmptyRenderer {}

impl Default for EmptyRenderer {
    fn default() -> Self {
        EmptyRenderer {  }
    }
}

impl Renderer for EmptyRenderer {
    fn current_tranform(&self) -> LocalToWorld {
        LocalToWorld::default()
    }
    fn begin(&mut self, capture: bool) {
        todo!()
    }

    fn transform(&mut self, transform: WorldToLocal) {
        todo!()
    }

    fn set_z_index(&mut self, z_index: i32) {
        todo!()
    }

    fn clip(&mut self, shape: Shape) {
        todo!()
    }

    fn clear_clip(&mut self) {
        todo!()
    }

    fn stroke(&mut self, shape: Shape, brush: Paint, width: f32) {
        todo!()
    }

    fn fill(&mut self, path: Shape, brush: Paint, blur_radius: f32) {
        todo!()
    }

    fn draw_text(&mut self, layout: &TextLayout, pos: LocalPoint) {
        todo!()
    }

    fn finish(&mut self) -> Option<DynamicImage> {
        todo!()
    }
}
