use nannou::prelude::*;

fn main() {
    nannou::app(model)
        .update(update)
        .run();
}

struct Model {
    size: u32,
    touch: bool
}

fn model(app: &App) -> Model {
    app.new_window()
        .size(600, 600)
        .view(view)
        .mouse_pressed(mouse_pressed)
        .mouse_released(mouse_released)
        .build()
        .unwrap();
    Model {
        size: 100,
        touch: false
    }
}

fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();
    draw.background().color(WHITE);
    draw.ellipse()
        .x_y(0.0, 0.0)
        .radius(model.size as f32)
        .rgb8(64, 64, 127);
    draw.to_frame(app, &frame).unwrap();
}

fn mouse_pressed(_: &App, model: &mut Model, _: MouseButton) {
    model.touch = true;
}

fn mouse_released(_: &App, model: &mut Model, _: MouseButton) {
    model.touch = false;
}

fn update(_: &App, model: &mut Model, _: Update) {
    if model.touch {
        model.size += 1;
        if model.size > 200 {
            model.size = 1;
        }
    }
}
