use rand::prelude::*;
use raylib::prelude::*;

use crate::utils::hypot_v;

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
    scene_size: Vector2,
}

impl Scene {
    pub fn new(window_width: i32, window_height: i32) -> Self {
        let mut rng: ThreadRng = rand::thread_rng();
        let obj_radius: f32 = rng.gen_range(100..200) as f32;
        let light_src_radius: f32 = rng.gen_range(20..100) as f32;

        Self {
            obj: Circle {
                center: Vector2::new(rng.gen_range(obj_radius..window_width as f32 - obj_radius), rng.gen_range(obj_radius..window_height as f32 - obj_radius)), 
                radius: obj_radius, 
                color: Color::LIGHTBLUE, 
            },
            light_src: Circle {
                center: Vector2::new(2.0 * light_src_radius, 2.0 * light_src_radius), 
                radius: light_src_radius, 
                color: Color::LIGHTYELLOW, 
            },
            scene_size: Vector2::new(window_width as f32, window_height as f32), 
        }
    }

    pub fn process(&mut self, mouse_pos: Vector2) {
        if !self.has_collisions(mouse_pos) {
            (self.light_src.center.x, self.light_src.center.y) = (mouse_pos.x, mouse_pos.y);
        };
    }

    pub fn draw(&mut self, d: &mut RaylibDrawHandle) {
        d.draw_circle_v(self.light_src.center, self.light_src.radius, self.light_src.color);
        d.draw_circle_v(self.obj.center, self.obj.radius, self.obj.color);
    }

    fn has_collisions(&self, pos: Vector2) -> bool {
        let dist: f32 = hypot_v(pos, self.obj.center);
        // ToDo: fix bug when mouse moves too fast and circles do not collide actually
        (dist <= self.light_src.radius + self.obj.radius) 
            || pos.x < 0.0 
            || pos.x > self.scene_size.x 
            || pos.y < 0.0 
            || pos.y > self.scene_size.y
    }
}