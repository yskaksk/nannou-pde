use nannou::prelude::*;

fn main() {
    nannou::sketch(view).run();
}

fn view(app: &App, frame: Frame) {
    if frame.nth() == 0 {
        let draw = app.draw();
        let d = 30.0;
        let win = app.window_rect();
        let x_counts = (win.w() / d).ceil() as i32;
        let y_counts = (win.h() / d).ceil() as i32;
        for i in 0..x_counts {
            let x = map_range(i, 0, x_counts-1, win.left() + d / 2.0, win.right() - d / 2.0);
            for j in 0..y_counts {
                let y = map_range(j, 0, y_counts-1, win.bottom() + d / 2.0, win.top() - d / 2.0);
                if random_f32() > 0.5 {
                    draw.tri()
                        .points(pt2(x - d / 2.0, y + d / 2.0), pt2(x - d / 2.0, y - d / 2.0), pt2(x + d / 2.0, y - d / 2.0))
                        .color(color_fn());
                    draw.tri()
                        .points(pt2(x - d / 2.0, y + d / 2.0), pt2(x + d / 2.0, y + d / 2.0), pt2(x + d / 2.0, y - d / 2.0))
                        .color(color_fn());
                } else {
                    draw.tri()
                        .points(pt2(x + d / 2.0, y + d / 2.0), pt2(x - d / 2.0, y + d / 2.0), pt2(x - d / 2.0, y - d / 2.0))
                        .color(color_fn());
                    draw.tri()
                        .points(pt2(x + d / 2.0, y + d / 2.0), pt2(x + d / 2.0, y - d / 2.0), pt2(x - d / 2.0, y - d / 2.0))
                        .color(color_fn());
                }
            }
        }
        draw.to_frame(app, &frame).unwrap();
    }
}

fn color_fn() -> rgb::Rgb {
    let green = random_range(0.0, 1.0);
    let red = random_range(0.0, green);
    let blue = random_range(0.0, red);
    return rgb::Rgb::new(red, green, blue)
}
