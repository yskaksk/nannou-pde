use nannou::prelude::*;

fn main() {
    nannou::sketch(view).run();
}

fn view(app: &App, frame: Frame) {
    let draw = app.draw();
    draw.background().color(WHITE);
    let win = app.window_rect();
    for i in 0..10 {
        let x = map_range(i, 0, 9, win.left(), win.right());
        for j in 0..10 {
            let y = map_range(j, 0, 9, win.bottom(), win.top());
            draw.ellipse().x_y(x, y).radius(10.0).color(BLACK);
        }
    }
    draw.to_frame(app, &frame).unwrap();
}
