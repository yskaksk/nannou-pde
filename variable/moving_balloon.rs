use nannou::prelude::*;

fn main() {
    nannou::app(model).run();
}

struct Model {
    x: f32,
    y: f32,
}

fn model(app: &App) -> Model {
    app.new_window()
        .size(600, 600)
        .view(view)
        .key_pressed(key_pressed)
        .build()
        .unwrap();
    Model { x: 0.0, y: 0.0 }
}

fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();
    draw.reset();
    draw.background().color(WHITE);
    draw.ellipse()
        .x_y(model.x, model.y)
        .radius(100.0)
        .rgb8(64, 32, 32);
    draw.to_frame(app, &frame).unwrap();
}

fn key_pressed(_: &App, model: &mut Model, key: Key) {
    let diff = 5.0;
    match key {
        Key::Left => model.x -= diff,
        Key::Right => model.x += diff,
        Key::Up => model.y += diff,
        Key::Down => model.y -= diff,
        _ => {}
    }
}
