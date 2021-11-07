use nannou::prelude::*;

fn main() {
    nannou::app(model).run();
}

struct Model {}

fn model(app: &App) -> Model {
    app.new_window()
        .size(600, 600)
        .title("animation/frame")
        .view(view)
        .build()
        .unwrap();
    Model {}
}

fn view(app: &App, _: &Model, frame: Frame) {
    let draw = app.draw();
    draw.reset();
    draw.background().color(WHITE);
    let time = app.time;
    let x = (time * 100.0) % 700.0 - 350.0;
    draw.ellipse()
        .x_y(x, 0.0)
        .w_h(100.0, 100.0)
        .rgb(0.5, 0.5, 0.8);
    draw.to_frame(app, &frame).unwrap();
}
