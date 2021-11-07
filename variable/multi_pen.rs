use nannou::prelude::*;

fn main() {
    nannou::app(model).run();
}

#[derive(Clone)]
struct Line {
    points: Vec<Point2>,
    mode: u8,
}

impl Line {
    fn push(&mut self, pp: Point2) {
        self.points.push(pp);
    }

    fn draw(&self, draw: &Draw) {
        let weight = if self.mode == 3 { 20.0 } else { 2.0 };
        draw.polyline().weight(weight).points_colored(
            self.points
                .iter()
                .map(|pp| (pt2(pp.x, pp.y), get_color(self.mode))),
        );
    }
}

fn get_color(mode: u8) -> Rgb<u8> {
    return if mode == 0 {
        BLACK
    } else if mode == 1 {
        GREEN
    } else if mode == 2 {
        RED
    } else {
        WHITE
    };
}

struct Model {
    figure: Vec<Line>,
    current_line: Option<Line>,
    mode: u8,
}

impl Model {
    fn undo(&mut self) {
        match self.current_line {
            Some(_) => self.current_line = None,
            None => {
                self.figure.pop();
            }
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
        .key_pressed(key_pressed)
        .build()
        .unwrap();
    Model {
        figure: vec![],
        current_line: None,
        mode: 0,
    }
}

fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();
    draw.background().color(WHITE);
    for l in model.figure.iter() {
        l.draw(&draw);
    }
    if let Some(current) = model.current_line.clone() {
        current.draw(&draw);
    }
    draw.ellipse()
        .x_y(280.0, 280.0)
        .radius(10.0)
        .color(get_color(model.mode));
    draw.to_frame(app, &frame).unwrap();
}

fn mouse_pressed(_: &App, model: &mut Model, button: MouseButton) {
    match button {
        MouseButton::Right => {
            model.mode += 1;
            if model.mode == 4 {
                model.mode = 0;
            }
        }
        MouseButton::Left => {
            model.current_line = Some(Line {
                points: vec![],
                mode: model.mode,
            });
        }
        _ => {}
    }
}

fn mouse_released(_: &App, model: &mut Model, button: MouseButton) {
    match button {
        MouseButton::Left => {
            if let Some(current) = model.current_line.clone() {
                model.figure.push(current);
            }
            model.current_line = None;
        }
        _ => {}
    }
}

fn mouse_moved(_: &App, model: &mut Model, pos: Point2) {
    if let Some(mut current) = model.current_line.clone() {
        current.push(pos);
        model.current_line = Some(current);
    }
}

fn key_pressed(app: &App, model: &mut Model, key: Key) {
    match key {
        Key::Q => app.quit(),
        Key::U => model.undo(),
        _ => {}
    }
}
