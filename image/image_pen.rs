use nannou::image::{self, GenericImageView};
use nannou::prelude::*;

fn main() {
    nannou::app(model).run();
}

struct Model {
    texture: wgpu::Texture,
    image: image::ImageBuffer<image::Rgb<u8>, Vec<u8>>,
    entered: bool,
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
        .mouse_entered(mouse_entered)
        .mouse_exited(mouse_exited)
        .build()
        .unwrap();
    Model {
        texture: wgpu::Texture::from_image(app, &img),
        image: img.to_rgb8(),
        entered: false,
        w,
        h,
    }
}

fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();
    if frame.nth() == 0 {
        draw.texture(&model.texture).x_y(0.0, 0.0);
    }
    let mouse = app.mouse;
    let wr = app.window_rect();
    let px = map_range(mouse.x.floor(), wr.x.start, wr.x.end, 0, model.w);
    let py = map_range(mouse.y.floor(), wr.y.start, wr.y.end, 0, model.h);
    let pixel = model.image.get_pixel(px, py);
    if model.entered {
        draw.ellipse()
            .x_y(mouse.x, mouse.y)
            .radius(10.0)
            .rgb8(pixel[0], pixel[1], pixel[2]);
    }
    draw.to_frame(app, &frame).unwrap();
}

fn mouse_entered(_: &App, model: &mut Model) {
    model.entered = true;
}

fn mouse_exited(_: &App, model: &mut Model) {
    model.entered = false;
}
