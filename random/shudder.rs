use nannou::prelude::*;

fn main() {
    nannou::app(model).run();
}

struct Model {}

fn model(app: &App) -> Model {
    app.new_window().size(600, 600).view(view).build().unwrap();
    Model {}
}

fn view(app: &App, _: &Model, frame: Frame) {
    let draw = app.draw();
    draw.background().rgb(0.8, 0.8, 0.8);
    draw.ellipse()
        .x_y(
            0.0 + random_range(-10.0, 10.0),
            0.0 + random_range(-10.0, 10.0),
        )
        .radius(150.0)
        .rgb8(
            34 + random_range(0, 60),
            64 + random_range(0, 60),
            4 + random_range(0, 60),
        );
    draw.to_frame(app, &frame).unwrap();
}
