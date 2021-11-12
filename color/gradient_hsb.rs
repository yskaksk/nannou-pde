use nannou::prelude::*;

fn main() {
    nannou::sketch(view).run();
}

fn view(app: &App, frame: Frame) {
    let draw = app.draw();
    draw.background().hsv(0.0, 0.0, 1.0);
    let win = app.window_rect();
    let n = 300;
    let red_hue = 0.0;
    let blue_hue = 240.0 / 360.0;
    for i in 0..n {
        let x = map_range(i, 0, n-1, win.left(), win.right());
        let hue = map_range(i, 0, n-1, red_hue, blue_hue);
        draw.rect()
            .x_y(x, 0.0)
            .w_h(win.w() / n as f32 + 1.0, win.h())
            .hsv(hue, 1.0, 1.0);
    }
    draw.to_frame(app, &frame).unwrap();
}
