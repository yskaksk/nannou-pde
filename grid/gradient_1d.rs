use nannou::prelude::*;

fn main() {
    nannou::sketch(view).run();
}

fn view(app: &App, frame: Frame) {
    let draw = app.draw();
    let win = app.window_rect();
    let n = 1000;
    for i in 0..n {
        let x = map_range(i, 0, n-1, win.right(), win.left());
        let cs = map_range(x, win.right(), win.left(), 0.0, 1.0);
        draw.rect()
            .x_y(x, 0.0)
            .w_h(win.w() / n as f32, win.h())
            .rgb(1.0 - cs, 0.0, cs);
    }
    draw.to_frame(app, &frame).unwrap();
}
