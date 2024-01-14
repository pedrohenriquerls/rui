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

    fn transform(&mut self, transform: LocalToWorld);

    fn set_z_index(&mut self, z_index: i32);

    /// Clip to a [`Shape`].
    fn clip(&mut self, shape: &Shape);

    fn clear_clip(&mut self);

    /// Stroke a [`Shape`].
    fn stroke<'b>(&mut self, shape: &Shape, brush: Paint, width: f32);

    /// Fill a [`Shape`], using the [non-zero fill rule].
    ///
    /// [non-zero fill rule]: https://en.wikipedia.org/wiki/Nonzero-rule
    fn fill<'b>(&mut self, path: &Shape, brush: Paint, blur_radius: f32);

    /// Draw a [`TextLayout`].
    ///
    /// The `pos` parameter specifies the upper-left corner of the layout object
    /// (even for right-to-left text).
    fn draw_text(&mut self, layout: &TextLayout, pos: LocalPoint);

    // fn draw_img(&mut self, img: Img<'_>, rect: LocalRect);

    fn finish(&mut self) -> Option<DynamicImage>;
}
