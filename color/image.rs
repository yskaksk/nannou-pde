use nannou::image::{self, GenericImageView};
use nannou::prelude::*;

use std::collections::HashMap;

fn main() {
    nannou::app(model).run();
}

struct Model {
    ratio: HashMap<(u8, u8, u8), f32>,
}

fn model(app: &App) -> Model {
    let assets = app.assets_path().unwrap();
    let img_path = assets.join("images").join("VanGogh.jpg");
    let img = image::open(img_path).unwrap();
    let mut colormap: HashMap<(u8, u8, u8), i32> = HashMap::new();
    let (w, h) = img.dimensions();
    let ib = img.to_rgb8();
    for x in 0..w {
        for y in 0..h {
            let pixel = ib.get_pixel(x, y);
            let c = (pixel[0], pixel[1], pixel[2]);
            if colormap.contains_key(&c) {
                if let Some(count) = colormap.get_mut(&c) {
                    *count += 1;
                }
            } else {
                colormap.insert(c, 1);
            }
        }
    }
    let mut sum = 0;
    for v in colormap.values() {
        sum += v;
    }
    let mut color_ratio: HashMap<(u8, u8, u8), f32> = HashMap::new();
    for (k, v) in colormap.into_iter() {
        color_ratio.insert(k, v as f32 / sum as f32);
    }
    app.new_window().size(800, 800).view(view).build().unwrap();
    return Model { ratio: color_ratio };
}

fn view(app: &App, model: &Model, frame: Frame) {
    if frame.nth() == 0 {
        let draw = app.draw();
        let d = 30.0;
        let win = app.window_rect();
        let x_counts = (win.w() / d).ceil() as i32;
        let y_counts = (win.h() / d).ceil() as i32;
        let color_fn = color_fn_gen(&model.ratio);
        for i in 0..x_counts {
            let x = map_range(
                i,
                0,
                x_counts - 1,
                win.left() + d / 2.0,
                win.right() - d / 2.0,
            );
            for j in 0..y_counts {
                let y = map_range(
                    j,
                    0,
                    y_counts - 1,
                    win.bottom() + d / 2.0,
                    win.top() - d / 2.0,
                );
                if random_f32() > 0.5 {
                    draw.tri()
                        .points(
                            pt2(x - d / 2.0, y + d / 2.0),
                            pt2(x - d / 2.0, y - d / 2.0),
                            pt2(x + d / 2.0, y - d / 2.0),
                        )
                        .color(color_fn());
                    draw.tri()
                        .points(
                            pt2(x - d / 2.0, y + d / 2.0),
                            pt2(x + d / 2.0, y + d / 2.0),
                            pt2(x + d / 2.0, y - d / 2.0),
                        )
                        .color(color_fn());
                } else {
                    draw.tri()
                        .points(
                            pt2(x + d / 2.0, y + d / 2.0),
                            pt2(x - d / 2.0, y + d / 2.0),
                            pt2(x - d / 2.0, y - d / 2.0),
                        )
                        .color(color_fn());
                    draw.tri()
                        .points(
                            pt2(x + d / 2.0, y + d / 2.0),
                            pt2(x + d / 2.0, y - d / 2.0),
                            pt2(x - d / 2.0, y - d / 2.0),
                        )
                        .color(color_fn());
                }
            }
        }
        draw.to_frame(app, &frame).unwrap();
    }
}

//fn color_fn(ratio: HashMap<(u8, u8, u8), f32>) -> rgb::Rgb {
//    let r = random_f32();
//    let mut s = 0.0;
//    for (k, v) in ratio.into_iter() {
//        s += v;
//        if r < s {
//            return rgb::Rgb::new(k.0 as f32 / 256.0, k.1 as f32 / 256.0, k.2 as f32 / 256.0)
//        }
//    }
//    return rgb::Rgb::new(1.0, 1.0, 1.0)
//}

fn color_fn_gen<'a>(ratio: &'a HashMap<(u8, u8, u8), f32>) -> impl Fn() -> rgb::Rgb + 'a {
    || {
        let r = random_f32();
        let mut s = 0.0;
        for (k, v) in ratio.clone().into_iter() {
            s += v;
            if r < s {
                return rgb::Rgb::new(k.0 as f32 / 256.0, k.1 as f32 / 256.0, k.2 as f32 / 256.0);
            }
        }
        return rgb::Rgb::new(1.0, 1.0, 1.0);
    }
}
