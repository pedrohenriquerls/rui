use rui::*;

fn main() {
    zstack((
        rectangle().color(AZURE_HIGHLIGHT).padding(Auto),
        circle().color(RED_HIGHLIGHT).padding(Auto),
    ))
    .run()
}
