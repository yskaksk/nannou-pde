use nannou::prelude::*;

fn main() {
    nannou::app(model).run();
}

struct Model {
    texture: wgpu::Texture,
}

fn model(app: &App) -> Model {
    app.new_window().size(800, 800).view(view).build().unwrap();
    let assets = app.assets_path().unwrap();
    let img_path = assets.join("images").join("VanGogh.jpg");
    Model {
        texture: wgpu::Texture::from_path(app, img_path).unwrap(),
    }
}

fn view(app: &App, model: &Model, frame: Frame) {
    frame.clear(BLACK);
    let draw = app.draw();
    let win = app.window_rect();
    let tile_count_x = map_range(app.mouse.x, win.left(), win.right(), 1.0, win.w() / 20.0);
    let tile_count_y = map_range(app.mouse.y, win.top(), win.bottom(), 1.0, win.h() / 20.0);
    let step_x = win.w() / tile_count_x;
    let step_y = win.h() / tile_count_y;

    for grid_y in (0..win.h() as usize).step_by(step_y as usize) {
        for grid_x in (0..win.w() as usize).step_by(step_x as usize) {
            let x = win.left() + grid_x as f32 + (step_x as f32 / 2.0);
            let y = win.top() - grid_y as f32 - (step_y as f32 / 2.0);
            draw.texture(&model.texture).x_y(x, y).w_h(step_x, step_y);
        }
    }
    draw.to_frame(app, &frame).unwrap();
}
