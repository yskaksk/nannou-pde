use nannou::image::{self, GenericImageView};
use nannou::prelude::*;

fn main() {
    nannou::app(model).run();
}

struct Model {
    image: image::ImageBuffer<image::Rgb<u8>, Vec<u8>>,
    w: u32,
    h: u32,
}

fn model(app: &App) -> Model {
    let assets = app.assets_path().unwrap();
    let img = image::open(assets.join("images").join("VanGogh.jpg")).unwrap();
    let (w, h) = img.dimensions();
    app.new_window()
        .size(w, h)
        .view(view)
        .build()
        .unwrap();
    Model {
        image: img.to_rgb8(),
        w, h
    }
}

fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();
    let win = app.window_rect();
    let n = 40;
    if frame.nth() == 0 {
        draw.background().color(BLACK);
    }
    for i in 0..n {
        let px = map_range(i, 0, n-1, 0, model.w-1);
        let x = map_range(i, 0, n-1, win.left(), win.right());
        for j in 0..n {
            let py = map_range(j, 0, n-1, 0, model.h-1);
            let y = map_range(j, 0, n-1, win.top(), win.bottom());
            let pixel = model.image.get_pixel(px, py);
            draw.ellipse()
                .x_y(x, y)
                .radius(5.0)
                .rgb8(pixel[0], pixel[1], pixel[2]);
        }
    }
    draw.to_frame(app, &frame).unwrap();
}
