use nannou::prelude::*;

fn main() {
    nannou::app(model).run();
}

struct Model {
    touch: bool,
}

impl Model {
    fn new() -> Self {
        Model { touch: false }
    }
}

fn model(app: &App) -> Model {
    app.new_window()
        .size(800, 800)
        .mouse_pressed(mouse_pressed)
        .mouse_released(mouse_released)
        .view(view)
        .build()
        .unwrap();
    Model::new()
}

fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();
    if frame.nth() == 0 {
        draw.background().rgb(0.1, 0.1, 0.1);
    }
    if model.touch {
        let mut x = app.mouse.x;
        let mut y = app.mouse.y;
        let mut i = 0;
        while i < 1000 {
            draw.ellipse()
                .x_y(x, y)
                .radius(2.0)
                .rgba(0.8, 0.8, 0.8, 0.1);
            x += random_range(-4.0, 4.0);
            y += random_range(-4.0, 4.0);
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
