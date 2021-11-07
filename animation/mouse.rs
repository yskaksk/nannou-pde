use nannou::prelude::*;

fn main() {
    nannou::app(model).run();
}

struct Model {}

fn model(app: &App) -> Model {
    app.new_window()
        .size(600, 600)
        .title("animation/mouse")
        .view(view)
        .build()
        .unwrap();
    Model {}
}

fn view(app: &App, _: &Model, frame: Frame) {
    let draw = app.draw();
    let mouse = app.mouse;
    draw.reset();
    draw.background().color(WHITE);
    draw.ellipse()
        .x_y(mouse.x, mouse.y)
        .w_h(100.0, 100.0)
        .rgb(0.5, 0.5, 0.8);
    draw.to_frame(app, &frame).unwrap();
}
