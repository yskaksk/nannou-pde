use nannou::prelude::*;

fn main() {
    nannou::sketch(view).run();
}

fn view(app: &App, frame: Frame) {
    let draw = app.draw();
    let win = app.window_rect();
    let n = 100;
    for i in 0..n {
        let x = map_range(i, 0, n-1, win.left(), win.right());
        let cs_x = map_range(x, win.left(), win.right(), 0.0, 1.0);
        for j in 0..n {
            let y = map_range(j, 0, n-1, win.bottom(), win.top());
            let cs_y = map_range(y, win.bottom(), win.top(), 0.0, 1.0);
            draw.rect()
                .x_y(x, y)
                .w_h(win.w() / n as f32, win.h() / n as f32)
                .rgb(cs_x, 0.0, cs_y);
        }
    }
    draw.to_frame(app, &frame).unwrap();
}
