use nannou::prelude::*;

fn main() {
    nannou::app(model).run();
}

struct Model {
    touch: bool,
    clear: bool
}

impl Model {
    fn new() -> Self {
        Model {
            touch: false,
            clear: false
        }
    }
}

fn model(app: &App) -> Model {
    app.new_window()
        .size(600, 600)
        .view(view)
        .mouse_pressed(mouse_pressed)
        .mouse_released(mouse_released)
        .key_pressed(key_pressed)
        .key_released(key_released)
        .build()
        .unwrap();
    Model::new()
}

fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();
    if frame.nth() == 0 {
        draw.background().rgb(0.8, 0.8, 0.8);
    }
    if model.clear {
        draw.reset();
        draw.background().rgb(0.8, 0.8, 0.8);
    }
    if model.touch {
        let x = app.mouse.x;
        let y = app.mouse.y;
        draw.ellipse()
            .x_y(x + random_range(-10.0, 10.0), y + random_range(-10.0, 10.0))
            .radius(random_range(1.0, 10.0))
            .color(BLACK);
    }
    draw.to_frame(app, &frame).unwrap();
}

fn mouse_pressed(_: &App, model: &mut Model, _: MouseButton) {
    model.touch = true;
}

fn mouse_released(_: &App, model: &mut Model, _: MouseButton) {
    model.touch = false;
}

fn key_pressed(_: &App, model: &mut Model, key: Key) {
    match key {
        Key::C => model.clear = true,
        _ => {}
    }
}
fn key_released(_: &App, model: &mut Model, key: Key) {
    match key {
        Key::C => model.clear = false,
        _ => {}
    }
}
