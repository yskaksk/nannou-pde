use nannou::prelude::*;

fn main() {
    nannou::app(model).run();
}

#[derive(Clone)]
struct Line {
    points: Vec<Point2>,
    button: Option<MouseButton>,
}

impl Line {
    fn new(button: MouseButton) -> Self {
        Line {
            points: vec![],
            button: Some(button),
        }
    }

    fn push(&mut self, pp: Point2) {
        self.points.push(pp);
    }

    fn draw(&self, draw: &Draw) {
        if let Some(button) = self.button {
            let color = match button {
                MouseButton::Left => RED,
                MouseButton::Right => BLACK,
                _ => unimplemented!(),
            };
            draw.polyline()
                .weight(2.0)
                .points_colored(self.points.iter().map(|pp| (pt2(pp.x, pp.y), color)));
        }
    }
}

struct Model {
    touch: bool,
    figure: Vec<Line>,
    current: Line,
}

impl Model {
    fn new() -> Self {
        Model {
            touch: false,
            figure: vec![],
            current: Line::new(MouseButton::Left),
        }
    }
}

fn model(app: &App) -> Model {
    app.new_window()
        .size(600, 600)
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
    draw.background().color(WHITE);
    for line in model.figure.iter() {
        line.draw(&draw);
    }
    model.current.draw(&draw);
    draw.to_frame(app, &frame).unwrap();
}

fn mouse_pressed(_: &App, model: &mut Model, button: MouseButton) {
    model.touch = true;
    model.current = Line::new(button);
}

fn mouse_released(_: &App, model: &mut Model, _: MouseButton) {
    model.touch = false;
    model.figure.push(model.current.clone());
    model.current = Line {
        points: vec![],
        button: None,
    }
}

fn mouse_moved(_: &App, model: &mut Model, pos: Point2) {
    if model.touch {
        model.current.push(pos);
    }
}

fn key_released(_: &App, model: &mut Model, key: Key) {
    match key {
        Key::C => {
            model.figure = vec![];
        }
        _ => {}
    }
}
