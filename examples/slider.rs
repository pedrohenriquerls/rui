use std::ops::Range;

use cosmic_text::{ AttrsList, Attrs, Style};
use peniko::Color;
use rui::*;

#[derive(Default)]
struct MyState {
    value: f32,
}

/// A slider with a value.
fn my_slider(s: impl Binding<f32>) -> impl View {
    with_ref(s, move |v| {
        let attrs = Attrs::new().color(Color::BLUE_VIOLET).style(Style::Italic);
        let attrs_list = AttrsList::new(attrs);
        vstack((
            v.to_string().layout(attrs_list).padding(Auto),
            hslider(s).thumb_color(RED_HIGHLIGHT).padding(Auto),
        )).background(rectangle().color(AZURE_HIGHLIGHT))
    })
}

fn main() {
    state(MyState::default, |state_handle, cx| {
        map(
            cx[state_handle].value,
            move |v, cx| cx[state_handle].value = v,
            |s, _| my_slider(s),
        )
    })
    .run()
}
