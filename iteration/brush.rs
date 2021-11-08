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
        .size(800, 800)
        .mouse_pressed(mouse_pressed)
        .mouse_released(mouse_released)
        .key_pressed(key_pressed)
        .key_released(key_released)
        .view(view)
        .build()
        .unwrap();
    Model::new()
}

fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();
    if model.clear {
        draw.reset();
        draw.background().rgb(0.8, 0.8, 0.8);
    }
    if frame.nth() == 0 {
        draw.background().rgb(0.8, 0.8, 0.8);
    }
    if model.touch {
        let mouse = app.mouse;
        let x = mouse.x;
        let y = mouse.y;
        let mut i = 0;
        while i < 10 {
            let size = random_range(1.0, 10.0);
            let alpha = random_range(0.0, 1.0);
            draw.ellipse()
                .x_y(x + random_range(-20.0, 20.0), y + random_range(-20.0, 20.0))
                .radius(size)
                .rgba(0.1, 0.1, 0.1, alpha);
            i += 1;
        }
    }
    draw.to_frame(app, &frame).unwrap();
}

fn mouse_pressed(_: &App, model: &mut Model, _: MouseButton) {
    model.touch = true;
}
fn mouse_released(_: &App, model: &mut Model, _: MouseButton) {
    model.touch = false;
}
fn key_pressed(app: &App, model: &mut Model, key: Key) {
    match key {
        Key::C => model.clear = true,
        Key::Q => app.quit(),
        _ => {}
    }
}
fn key_released(_: &App, model: &mut Model, key: Key) {
    match key {
        Key::C => model.clear = false,
        _ => {}
    }
}
