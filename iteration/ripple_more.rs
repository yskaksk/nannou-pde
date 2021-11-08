use nannou::prelude::*;

fn main() {
    nannou::app(model).run();
}

struct Model {
    points: Vec<Point2>,
}

fn model(app: &App) -> Model {
    app.new_window()
        .mouse_pressed(mouse_pressed)
        .view(view)
        .build()
        .unwrap();
    Model { points: vec![] }
}

fn mouse_pressed(app: &App, model: &mut Model, _: MouseButton) {
    let mouse = app.mouse;
    model.points.push(pt2(mouse.x, mouse.y));
}

fn circle(draw: &Draw, x: f32, y: f32, radius: f32) {
    let points = (0..=60).map(|i| {
        let rad = deg_to_rad(6.0 * i as f32);
        let x_loc = rad.sin() * radius + x;
        let y_loc = rad.cos() * radius + y;
        (pt2(x_loc, y_loc), BLACK)
    });
    draw.polyline().weight(2.0).points_colored(points);
}

fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();
    draw.background().color(WHITE);
    for pt in model.points.iter() {
        for i in 0..=20 {
            circle(&draw, pt.x, pt.y - 20.0 * i as f32, 10.0 * i as f32);
        }
    }
    draw.to_frame(app, &frame).unwrap();
}
