use rand::prelude::*;
use raylib::prelude::*;

struct Circle {
    center: Vector2,
    radius: f32,
    color: Color,
}

struct Ray {
    pos: Vector2,
    angle: f32,
}

pub struct Scene {
    obj: Circle,
    light_src: Circle, 
}

impl Scene {
    pub fn new(window_width: i32, window_height: i32) -> Self {
        let mut rng: ThreadRng = rand::thread_rng();
        let obj_radius: f32 = rng.gen_range(50..200) as f32;
        let light_src_radius: f32 = rng.gen_range(20..100) as f32;

        Self {
            obj: Circle {
                center: Vector2::new(rng.gen_range(obj_radius..window_width as f32 - obj_radius), rng.gen_range(obj_radius..window_height as f32 - obj_radius)), 
                radius: obj_radius, 
                color: Color::LIGHTBLUE, 
            },
            light_src: Circle {
                center: Vector2::new(-2.0 * light_src_radius, -2.0 * light_src_radius), 
                radius: light_src_radius, 
                color: Color::LIGHTYELLOW, 
            },
        }
    }

    pub fn draw(&mut self, d: &mut RaylibDrawHandle, mouse_pos: Vector2) {
        // Update light source coordinates
        (self.light_src.center.x, self.light_src.center.y) = (mouse_pos.x, mouse_pos.y);

        // Drawing
        if self.light_src.center.x > 0.0 && self.light_src.center.y > 0.0 {
            d.draw_circle_v(self.light_src.center, self.light_src.radius, self.light_src.color);
        }
        d.draw_circle_v(self.obj.center, self.obj.radius, self.obj.color);
    }
}