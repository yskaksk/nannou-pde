use nannou::prelude::*;

fn main() {
    nannou::app(model).run();
}

struct Circle {
    x: f32,
    y: f32,
    color: Rgb<u8>
}

impl Circle {
    fn draw(&self, draw: &Draw) {
        draw.ellipse()
            .x_y(self.x, self.y)
            .w_h(50.0, 50.0)
            .color(self.color);
    }
}

struct Model {
    touch: bool,
    figures: Vec<Circle>
}

impl Model {
    fn new() -> Self {
        Model {
            touch: false,
            figures: vec![]
        }
    }

    fn clear(&mut self) {
        self.figures = vec![];
    }
}

fn model(app: &App) -> Model {
    app.new_window()
        .size(600, 600)
        .title("second pen")
        .view(view)
        .mouse_pressed(mouse_pressed)
        .mouse_released(mouse_released)
        .mouse_moved(mouse_moved)
        .key_released(key_released)
        .build()
        .unwrap();
    Model::new()
}

fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();
    draw.background().rgb(0.8, 0.8, 0.8);
    for c in model.figures.iter() {
        c.draw(&draw);
    }
    draw.to_frame(app, &frame).unwrap();
}

fn mouse_pressed(_: &App, model: &mut Model, _: MouseButton) {
    model.touch = true;
}

fn mouse_released(_: &App, model: &mut Model, _: MouseButton) {
    model.touch = false;
}

fn mouse_moved(app: &App, model: &mut Model, pos: Point2) {
    if model.touch {
        let time = (app.time * 1000.0).floor() as i32;
        let color = if time % 2 == 0 {
            RED
        } else {
            WHITE
        };
        model.figures.push(Circle {
            x: pos.x,
            y: pos.y,
            color
        })
    }
}

fn key_released(_: &App, model: &mut Model, key: Key) {
    match key {
        Key::C => {
            model.clear();
        }
        _ => {}
    }
}
