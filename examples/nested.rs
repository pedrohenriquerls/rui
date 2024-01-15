use rui::*;

fn my_rectangle(color: vger::Color) -> impl View {
    rectangle()
        .corner_radius(30.0)
        .color(color)
        .padding(Auto)
}

fn main() {
    hstack((
        my_rectangle(RED_HIGHLIGHT),
        vstack((
            my_rectangle(AZURE_HIGHLIGHT),
            hstack((
                my_rectangle(BUTTON_HOVER_COLOR),
                vstack((
                    my_rectangle(AZURE_HIGHLIGHT_DARK),
                    hstack((my_rectangle(RED_HIGHLIGHT_BACKGROUND), vstack((my_rectangle(RED_HIGHLIGHT), my_rectangle(RED_HIGHLIGHT))))),
                )),
            )),
        )),
    ))
    .run()
}
