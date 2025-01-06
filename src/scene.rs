use rand::prelude::*;
use raylib::prelude::*;
use std::f64::consts::PI as PI;

use crate::utils::hypot_v;

const RAYS_AMOUNT: i32 = 200;

struct Circle {
    center: Vector2,
    radius: f32,
    color: Color,
}

struct Ray {
    angle: f32,
    pos: Vector2,
}

pub struct Scene {
    objects: Vec<Circle>,
    light_src: Circle, 
    rays: Vec<Ray>, 
    max_ray_dist: f32, 
    scene_size: Vector2, 
}

impl Scene {
    pub fn new(window_width: i32, window_height: i32) -> Self {
        let mut rng: ThreadRng = rand::thread_rng();
        let obj_radius: f32 = rng.gen_range(100..200) as f32;
        let light_src_radius: f32 = rng.gen_range(20..100) as f32;
        let mut ray_angle: f32;

        let mut scene: Self = Self {
            objects: Vec::with_capacity(10),
            light_src: Circle {
                center: Vector2::new(2.0 * light_src_radius, 2.0 * light_src_radius), 
                radius: light_src_radius, 
                color: Color::LIGHTYELLOW, 
            },
            rays: Vec::with_capacity(RAYS_AMOUNT as usize),
            max_ray_dist: ((window_width * window_width + window_height * window_height) as f32).sqrt(),
            scene_size: Vector2::new(window_width as f32, window_height as f32), 
        };

        scene.objects.push(Circle {
            center: Vector2::new(rng.gen_range(obj_radius..window_width as f32 - obj_radius), rng.gen_range(obj_radius..window_height as f32 - obj_radius)), 
            radius: obj_radius, 
            color: Color::LIGHTBLUE, 
        });
        for i in 0..RAYS_AMOUNT {
            ray_angle = (i as f32 / RAYS_AMOUNT as f32) * 2.0 * PI as f32;
            scene.rays.push(Ray {
                angle: ray_angle,
                pos: Vector2 {
                    x: scene.max_ray_dist * f32::cos(ray_angle) + scene.light_src.center.x, 
                    y: scene.max_ray_dist * f32::sin(ray_angle) + scene.light_src.center.y,
                },
            });
        }

        scene
    }

    pub fn update_light_source_pos(&mut self, mouse_pos: Vector2) {
        if !self.has_collisions(mouse_pos) {
            self.light_src.center = mouse_pos;
            self.gen_rays();
        };
    }

    pub fn draw(&mut self, d: &mut RaylibDrawHandle) {
        d.draw_circle_v(self.light_src.center, self.light_src.radius, self.light_src.color);

        for ray in self.rays.iter() {
            d.draw_line_v(Vector2 {
                x: self.light_src.radius * f32::cos(ray.angle) + self.light_src.center.x, 
                y: self.light_src.radius * f32::sin(ray.angle) + self.light_src.center.y,
            }, ray.pos, Color::YELLOW);
        }
        
        for item in self.objects.iter() {
            d.draw_circle_v(item.center, item.radius, item.color);
        }
    }

    // Private methods

    fn has_collisions(&self, pos: Vector2) -> bool {
        if pos.x < 0.0 || pos.x > self.scene_size.x || pos.y < 0.0 || pos.y > self.scene_size.y {
            return true;
        }
        for item in self.objects.iter() {
            if hypot_v(pos, item.center) <= self.light_src.radius + item.radius {
                return true;
            }
        }
        return false;
    }

    fn gen_rays(&mut self) {
        let mut dist_pline: f32;
        let mut dist: f32;

        for ray in self.rays.iter_mut() {
            for obj in self.objects.iter() {
                dist_pline = Self::get_point_paline_distance_v(obj.center, ray.pos, ray.angle);
                if dist_pline < obj.radius {
                    dist = hypot_v(self.light_src.center, obj.center) - obj.radius;
                    if hypot_v(Vector2 {
                        x: dist * f32::cos(ray.angle) + self.light_src.center.x,
                        y: dist * f32::sin(ray.angle) + self.light_src.center.y,
                    }, obj.center) > dist + obj.radius {
                        dist = self.max_ray_dist
                    } else {
                        dist = ((dist + obj.radius).powi(2) - dist_pline.powi(2)).sqrt() - (obj.radius.powi(2) - dist_pline.powi(2)).sqrt()
                    }
                } else {
                    dist = self.max_ray_dist;
                }
                
                ray.pos.x = dist * f32::cos(ray.angle) + self.light_src.center.x;
                ray.pos.y = dist * f32::sin(ray.angle) + self.light_src.center.y;
            }
        }
    }

    fn get_point_paline_distance_v(p: Vector2, lp: Vector2, angle: f32) -> f32 {
        (f32::cos(angle) * (lp.y - p.y) - f32::sin(angle) * (lp.x - p.x)).abs()
    }
}