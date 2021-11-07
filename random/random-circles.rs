use nannou::prelude::*;


fn main() {
    nannou::app(model).run();
}

struct Model {}

fn model(app: &App) -> Model {
    let _ = app.new_window()
        .size(600, 600)
        .view(view)
        .build()
        .unwrap();
    Model {}
}

fn view(app: &App, _: &Model, frame: Frame) {
    let draw = app.draw();
    draw.ellipse()
        .x_y(random_range(-300.0, 300.0), random_range(-300.0, 300.0))
        .w_h(random_range(0.0, 200.0), random_range(0.0, 200.0))
        .rgba8(random_range(0, 127), random_range(0, 127), random_range(0, 127), random_range(110, 127));
    draw.to_frame(app, &frame).unwrap();
}
