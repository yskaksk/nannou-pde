use nannou::prelude::*;

fn main() {
    nannou::sketch(view).run();
}

fn view(app: &App, frame: Frame) {
    let draw = app.draw();
    let win = app.window_rect();
    draw.background().color(WHITE);
    for i in 0..10 {
        let x = map_range(i, 0, 9, win.left(), win.right());
        let cs = map_range(x, 0.0, win.right(), 0.0, 255.0);
        draw.ellipse().x_y(x, 0.0).radius(10.0).rgb8(
            cs.floor() as u8,
            0,
            (255.0 - cs).floor() as u8,
        );
    }
    draw.to_frame(app, &frame).unwrap();
}
