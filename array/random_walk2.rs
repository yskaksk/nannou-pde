use nannou::prelude::*;

fn main() {
    nannou::app(model).update(update).run();
}

struct Agent {
    x: f32,
    y: f32
}

impl Agent {
    fn new() -> Self {
        Agent {
            x: 0.0,
            y: 0.0
        }
    }

    fn update(&mut self) {
        let d = 1.0;
        let theta = random_f32();
        if theta < 0.25 {
            self.x += d;
        } else if theta < 0.5 {
            self.y += d;
        } else if theta < 0.75 {
            self.x -= d;
        } else {
            self.y -= d;
        }
    }

    fn draw(&self, draw: &Draw) {
        draw.ellipse()
            .x_y(self.x, self.y)
            .radius(5.0)
            .color(BLACK);
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
        .build()
        .unwrap();
    let n = 100;
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
