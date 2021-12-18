use nannou::prelude::*;

fn main() {
    nannou::app(model).update(update).run();
}

struct Agent {
    x: f32,
    y: f32,
    size: f32,
    color: rgb::Rgb
}

impl Agent {
    fn new() -> Self {
        let size = random_range(1.0, 10.0);
        let r = random_range(0.0, 1.0);
        let g = random_range(0.2, 0.8);
        let b = random_range(0.6, 1.0);
        Agent {
            x: 0.0,
            y: 0.0,
            size,
            color: rgb::Rgb::new(r, g, b)
        }
    }

    fn update(&mut self) {
        let d = self.size / 4.0;
        let theta = random_f32();
        if theta < 0.25 {
            self.x += d;
            self.size = (self.size + 1.0).min(20.0);
        } else if theta < 0.5 {
            self.y += d;
            self.size = (self.size - 1.0).max(0.0);
        } else if theta < 0.75 {
            self.x -= d;
            self.size = (self.size + 1.0).min(20.0);
        } else {
            self.y -= d;
            self.size = (self.size - 1.0).max(0.0);
        }
    }

    fn draw(&self, draw: &Draw) {
        draw.ellipse()
            .x_y(self.x, self.y)
            .radius(self.size)
            .color(self.color);
    }
}

struct Model {
    agents: Vec<Agent>
}

impl Model {
    fn update(&mut self) {
        for a in self.agents.iter_mut() {
            a.update();
        }
    }

    fn draw(&self, draw: &Draw) {
        for a in self.agents.iter() {
            a.draw(draw);
        }
    }
}

fn model(app: &App) -> Model {
    app.new_window()
        .view(view)
        .size(600, 600)
        .build()
        .unwrap();
    let n = 1000;
    let mut agents: Vec<Agent> = Vec::with_capacity(n);
    for _ in 0..n {
        agents.push(Agent::new());
    }
    Model {
        agents
    }
}

fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();
    draw.background().color(WHITE);
    model.draw(&draw);
    draw.to_frame(app, &frame).unwrap();
}

fn update(_app: &App, model: &mut Model, _update: Update) {
    model.update();
}
