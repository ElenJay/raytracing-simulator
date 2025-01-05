use raylib::prelude::*;

mod scene;
mod utils;

use scene::Scene;

const WINDOW_WIDTH: i32 = 1600;
const WINDOW_HEIGHT: i32 = 900;

fn main() {
    let (mut rl, thread) = raylib::init()
        .size(WINDOW_WIDTH, WINDOW_HEIGHT)
        .title("Raytracing Simulator")
        .vsync()
        .build();

    let mut scene: Scene = Scene::new(WINDOW_WIDTH, WINDOW_HEIGHT);
     
    while !rl.window_should_close() {
        let mouse_pos = rl.get_mouse_position();
        scene.process(mouse_pos);

        let mut d = rl.begin_drawing(&thread); 
        d.clear_background(Color::BLACK);

        scene.draw(&mut d);
        draw_text_center(&mut d, "Move circle (the source light) with your mouse!", 12, 36, Color::WHITE);
    }
}

fn draw_text_center(d: &mut RaylibDrawHandle, text: &str, y: i32, font_size: i32, color: Color) {
    let text_length = d.measure_text(text, font_size);
    d.draw_text(text, (WINDOW_WIDTH - text_length) / 2, y, font_size, color);
}