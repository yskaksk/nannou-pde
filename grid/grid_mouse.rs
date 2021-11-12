use nannou::prelude::*;

fn main() {
    nannou::sketch(view).run();
}

fn view(app: &App, frame: Frame) {
    let draw = app.draw();
    draw.background().color(WHITE);
    let win = app.window_rect();
    let mouse = pt2(app.mouse.x, app.mouse.y);
    let n = 20;
    for i in 0..n {
        let x = map_range(i, 0, n-1, win.left(), win.right());
        for j in 0..n {
            let y = map_range(j, 0, n-1, win.bottom(), win.top());
            let d = mouse.distance(pt2(x, y));
            draw.ellipse()
                .x_y(x, y)
                .radius(d * 0.05)
                .color(BLACK);
        }
    }
    draw.to_frame(app, &frame).unwrap();
}
