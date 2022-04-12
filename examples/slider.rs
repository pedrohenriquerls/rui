use rui::*;

#[derive(Clone, Default)]
struct MyState {
    value: f32,
}

make_lens!(ValueLens, MyState, f32, value);

fn main() {
    rui(state(MyState::default, |state, cx| {
        vstack((
            text(&format!("value: {:?}", cx[state].value))
                .font_size(10)
                .padding(Auto),
            hslider(bind(state, ValueLens {}))
                .thumb_color(RED_HIGHLIGHT)
                .padding(Auto),
        ))
    }));
}
