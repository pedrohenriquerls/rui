use rui::*;

#[derive(Default)]
struct MyState {
    value: f32,
}

make_lens!(ValueLens, MyState, f32, value);

fn main() {
    state(MyState::default, |state, cx| {
        vstack((
            text(&cx[state].value.to_string()).padding(Auto),
            hslider(bind(state, ValueLens {}))
                .thumb_color(RED_HIGHLIGHT)
                .padding(Auto),
        ))
    })
    .run()
}
