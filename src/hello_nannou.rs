use nannou::prelude::*;

fn main() {
    nannou::app(model).run();
}

struct Model;

fn model(app: &App) -> Model {
    let _ = app.new_window().size(600, 600).view(view).build().unwrap();
    Model {}
}

fn view(app: &App, _model: &Model, frame: Frame) {
    let draw = app.draw();
    draw.background().rgb(0.2, 0.2, 0.4);
    draw.ellipse()
        .x_y(0.0, 0.0)
        .w_h(400.0, 400.0)
        .rgb(0.7, 0.7, 0.7)
        .stroke(BLACK);

    draw.ellipse().x_y(70.0, 70.0).w_h(45.0, 45.0).color(BLACK);
    draw.ellipse().x_y(80.0, 80.0).w_h(15.0, 15.0).color(WHITE);
    draw.ellipse().x_y(-70.0, 70.0).w_h(45.0, 45.0).color(BLACK);
    draw.ellipse().x_y(-60.0, 80.0).w_h(15.0, 15.0).color(WHITE);
    draw.ellipse().x_y(0.0, -70.0).w_h(80.0, 80.0).color(RED);

    draw.to_frame(app, &frame).unwrap();
}
