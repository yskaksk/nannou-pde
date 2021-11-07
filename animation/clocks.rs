use nannou::prelude::*;
use std::f32;

fn main() {
    nannou::app(model).update(update).run();
}

struct Model {
    hour: u64,
    minute: u64,
    second: u64
}

fn model(app: &App) -> Model {
    app.new_window()
        .size(600, 600)
        .title("animation/frame")
        .view(view)
        .build()
        .unwrap();
    Model {
        hour: 0,
        minute: 0,
        second: 0
    }
}

fn view(app: &App, model: &Model, frame: Frame) {
    let pi = f32::consts::PI;
    let draw = app.draw();
    draw.reset();
    draw.background().color(WHITE);
    let h_rad = -1.0 * (model.hour + 9) as f32 * pi / 6.0;
    let m_rad = -1.0 * (model.minute + 45) as f32 * pi / 30.0;
    let s_rad = -1.0 * (model.second + 45) as f32 * pi / 30.0;
    draw.ellipse()
        .x_y(175.0 * h_rad.cos(), 175.0 * h_rad.sin())
        .w_h(200.0, 200.0)
        .rgb(0.1, 0.3, 0.8);
    draw.ellipse()
        .x_y(125.0 * m_rad.cos(), 125.0 * m_rad.sin())
        .w_h(100.0, 100.0)
        .rgb(0.2, 0.4, 0.2);
    draw.ellipse()
        .x_y(100.0 * s_rad.cos(), 100.0 * s_rad.sin())
        .w_h(50.0, 50.0)
        .rgb(0.9, 0.2, 0.4);
    draw.to_frame(app, &frame).unwrap();
}

fn update(_: &App, model: &mut Model, update: Update) {
    let duration = update.since_start.as_secs();
    model.hour = duration / 3600;
    model.minute = (duration % 3600) / 60;
    model.second = duration % 60;
}
