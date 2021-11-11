use nannou::prelude::*;

fn main() {
    nannou::app(model).view(view).run();
}

struct Model {
    texture_left: wgpu::Texture,
    texture_right: wgpu::Texture,
}

fn model(app: &App) -> Model {
    app.new_window().size(800, 800).build().unwrap();
    let assets = app.assets_path().unwrap();
    let img_path_l = assets.join("images").join("stamp.png");
    let img_path_r = assets.join("images").join("stamp2.png");
    Model {
        texture_left: wgpu::Texture::from_path(app, img_path_l).unwrap(),
        texture_right: wgpu::Texture::from_path(app, img_path_r).unwrap(),
    }
}

fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();
    if frame.nth() == 0 {
        draw.background().color(WHITE);
    }
    let mouse = app.mouse;
    if mouse.buttons.left().is_down() {
        draw.texture(&model.texture_left)
            .x_y(mouse.x, mouse.y)
            .w_h(100.0, 100.0);
    }
    if mouse.buttons.right().is_down() {
        draw.texture(&model.texture_right)
            .x_y(mouse.x, mouse.y)
            .w_h(100.0, 100.0);
    }
    draw.to_frame(app, &frame).unwrap();
}
