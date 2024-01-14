use crate::*;

const SLIDER_WIDTH: f32 = 4.0;
const SLIDER_THUMB_RADIUS: f32 = 10.0;

#[derive(Clone, Copy)]
pub struct SliderOptions {
    thumb: Color,
}

impl Default for SliderOptions {
    fn default() -> Self {
        Self {
            thumb: AZURE_HIGHLIGHT,
        }
    }
}

pub trait SliderMods: View + Sized {
    fn thumb_color(self, color: Color) -> Self;
}

/// Horizontal slider built from other Views.
pub fn hslider(value: impl Binding<f32>) -> impl SliderMods {
    modview(move |opts: SliderOptions, _| {
        state(
            || 0.0,
            move |width, cx| {
                let w = cx[width];
                canvas(move |args, sz| {
                    let c = sz.center();

                    let w = args.cx[width];
                    let v = value.get(args.cx);
                    let r = SLIDER_THUMB_RADIUS;
                    let start_x = r;
                    let end_x = w - r;
                    let x = (1.0 - v) * start_x + v * (end_x);
                    let rect = LocalRect::new(
                        [ start_x, c.y - SLIDER_WIDTH / 2.0 ].into(),
                        [sz.size.width - 2.0 * r, SLIDER_WIDTH].into()
                    );
                    args.rd.fill(
                        Shape::Rectangle(&rect, 0.0),
                        Paint::Color(BUTTON_BACKGROUND_COLOR),
                        0.0
                    );

                    let paint = Paint::Color(AZURE_HIGHLIGHT_BACKGROUND);
                    let rect = LocalRect::new([ start_x, c.y - SLIDER_WIDTH / 2.0 ].into(), [ x, SLIDER_WIDTH ].into());
                    args.rd.fill(
                        Shape::Rectangle(&rect, 0.0),
                        paint,
                        0.0,
                    );

                    args.rd.fill(Shape::Circle(&LocalPoint::new(x, c.y), r), Paint::Color(opts.thumb), 0.0);
                    args.rd.fill(Shape::Background, Paint::Color(AZURE_HIGHLIGHT_BACKGROUND), 0.0);
                })
                .geom(move |cx, sz, _| {
                    if sz.width != cx[width] {
                        cx[width] = sz.width;
                    }
                })
                .drag_s(value, move |v, delta, _, _| {
                    *v = (*v + delta.x / w).clamp(0.0, 1.0)
                })
            },
        )
        .role(accesskit::Role::Slider)
    })
}

impl<F> SliderMods for ModView<SliderOptions, F>
where
    ModView<SliderOptions, F>: View,
{
    fn thumb_color(self, color: Color) -> Self {
        let mut opts = self.value;
        opts.thumb = color;
        ModView {
            func: self.func,
            value: opts,
        }
    }
}

/// Vertical slider built from other Views.
pub fn vslider(
    value: f32,
    set_value: impl Fn(&mut Context, f32) + 'static + Copy,
) -> impl SliderMods {
    modview(move |opts: SliderOptions, _| {
        state(
            || 0.0,
            move |height, _| {
                canvas(move |args, sz| {
                    let h = args.cx[height];
                    let y = value * h;
                    let c = sz.center();
                    let paint = Paint::Color(BUTTON_BACKGROUND_COLOR);
                    let rect = LocalRect::new([ c.x - SLIDER_WIDTH / 2.0, 0.0 ].into(), [ SLIDER_WIDTH, sz.height() ].into());
                    args.rd.fill(
                        Shape::Rectangle(&rect, 0.0),
                        paint,
                        0.0,
                    );
                    args.rd.fill(Shape::Circle(&LocalPoint::new(c.x, y), SLIDER_THUMB_RADIUS), Paint::Color(opts.thumb), 0.0);
                })
                .geom(move |cx, sz, _| {
                    if sz.height != cx[height] {
                        cx[height] = sz.height;
                    }
                })
                .drag(move |cx, delta, _, _| {
                    (set_value)(cx, (value + delta.y / cx[height]).clamp(0.0, 1.0));
                })
            },
        )
    })
}
