use nannou::prelude::*;

fn main() {
    nannou::app(model).run();
}

struct Model {
    on: bool,
}

fn model(app: &App) -> Model {
    app.new_window()
        .size(600, 600)
        .mouse_pressed(mouse_pressed)
        .view(view)
        .build()
        .unwrap();
    Model { on: true }
}

fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();
    if model.on {
        draw.background().color(WHITE);
        draw.ellipse()
            .x_y(0.0, 0.0)
            .radius(180.0)
            .rgb8(255, 255, 224);
        draw.ellipse()
            .x_y(0.0, 0.0)
            .radius(150.0)
            .rgb8(240, 230, 140);
        draw.ellipse().x_y(0.0, 0.0).radius(120.0).rgb8(255, 215, 0);
    } else {
        draw.background().color(BLACK);
        draw.ellipse().x_y(0.0, 0.0).radius(150.0).rgb8(25, 25, 25);
    }
    draw.to_frame(app, &frame).unwrap();
}

fn mouse_pressed(_: &App, model: &mut Model, _: MouseButton) {
    model.on = !model.on;
}
