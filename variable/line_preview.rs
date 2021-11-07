use nannou::prelude::*;

fn main() {
    nannou::app(model).run();
}

struct Model {
    x: f32,
    y: f32,
    clicked: bool,
}

fn model(app: &App) -> Model {
    app.new_window()
        .size(600, 600)
        .mouse_pressed(mouse_pressed)
        .key_pressed(key_pressed)
        .view(view)
        .build()
        .unwrap();
    Model {
        x: 0.0,
        y: 0.0,
        clicked: false,
    }
}

fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();
    draw.background().color(WHITE);
    if model.clicked {
        let mouse = app.mouse;
        draw.ellipse()
            .x_y(model.x, model.y)
            .radius(5.0)
            .color(BLACK);
        draw.line()
            .start(pt2(model.x, model.y))
            .end(pt2(mouse.x, mouse.y));
    }
    draw.to_frame(app, &frame).unwrap();
}

fn mouse_pressed(app: &App, model: &mut Model, _: MouseButton) {
    let mouse = app.mouse;
    model.x = mouse.x;
    model.y = mouse.y;
    model.clicked = true;
}

fn key_pressed(app: &App, _: &mut Model, key: Key) {
    match key {
        Key::Q => app.quit(),
        _ => {}
    }
}
