use nannou::prelude::*;

fn main() {
    nannou::app(model).run();
}

#[derive(Clone)]
enum Mode {
    YX,
    Fixed,
    Gradient,
}

#[derive(Clone)]
struct Line {
    points: Vec<Point2>,
    mode: Mode,
}

impl Line {
    fn new(mode: Mode) -> Self {
        Line {
            points: vec![],
            mode: mode,
        }
    }

    fn push(&mut self, pp: Point2) {
        self.points.push(pp)
    }

    fn draw(&self, draw: &Draw) {
        match self.mode {
            Mode::YX => {
                let points = self.points.iter().map(|pp| (pt2(pp.x, pp.y), RED));
                let rev = self.points.iter().map(|pp| (pt2(pp.y, pp.x), BLUE));
                draw.polyline().weight(2.0).points_colored(points);
                draw.polyline().weight(2.0).points_colored(rev);
            }
            Mode::Fixed => {
                for pp in self.points.iter() {
                    draw.line().start(*pp).end(pt2(-pp.x, -pp.y));
                }
            }
            Mode::Gradient => {
                for pp in self.points.iter() {
                    draw.ellipse().x_y(pp.x, pp.y).w_h(20.0, 20.0).rgb(
                        0.5,
                        0.5,
                        (pp.x + 300.0) / 600.0,
                    );
                }
            }
        }
    }
}

struct Model {
    figure: Vec<Line>,
    current: Line,
    touch: bool,
    mode: Mode,
}

fn model(app: &App) -> Model {
    app.new_window()
        .size(600, 600)
        .view(view)
        .key_released(key_released)
        .mouse_pressed(mouse_pressed)
        .mouse_released(mouse_released)
        .mouse_moved(mouse_moved)
        .build()
        .unwrap();
    Model {
        figure: vec![],
        current: Line::new(Mode::YX),
        touch: false,
        mode: Mode::YX,
    }
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

fn mouse_pressed(_: &App, model: &mut Model, _: MouseButton) {
    model.touch = true;
}

fn mouse_released(_: &App, model: &mut Model, _: MouseButton) {
    model.touch = false;
    model.figure.push(model.current.clone());
    model.current = Line::new(model.mode.clone());
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
        Key::T => {
            match model.mode {
                Mode::YX => model.mode = Mode::Fixed,
                Mode::Fixed => model.mode = Mode::Gradient,
                Mode::Gradient => model.mode = Mode::YX,
            }
            model.current = Line::new(model.mode.clone());
        }
        _ => {}
    }
}
