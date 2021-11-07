use nannou::prelude::*;

fn main() {
    nannou::app(model).update(update).run();
}

struct Model {
    x: f32,
    y: f32,
}

fn model(app: &App) -> Model {
    app.new_window().size(600, 600).view(view).build().unwrap();
    Model { x: 0.0, y: 0.0 }
}

fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();
    draw.background().color(WHITE);
    draw.ellipse()
        .x_y(model.x, model.y)
        .radius(15.0)
        .color(BLACK);
    draw.to_frame(app, &frame).unwrap();
}

fn update(_: &App, model: &mut Model, _: Update) {
    let diff = 10.0;
    if random_f32() > 0.5 {
        model.x += diff;
    } else {
        model.x -= diff;
    }

    if random_f32() > 0.5 {
        model.y += diff;
    } else {
        model.y -= diff;
    }
}
