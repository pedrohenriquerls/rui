use rui::*;

fn main() {
    hstack((
        circle().color(RED_HIGHLIGHT)
            .tap(|_| {
                println!("circle tapped");
            })
            .padding(Auto),
        rectangle()
            .corner_radius(5.0)
            .color(AZURE_HIGHLIGHT)
            .tap(|_| { // actions must be limited to the shape that receive the event
                println!("rect tapped");
            })
            .padding(Auto),
    ))
    .run()
}
