use raylib::core::math::Vector2;

pub fn hypot_v(pos1: Vector2, pos2: Vector2) -> f32 {
    ((pos2.x - pos1.x).powi(2) + (pos2.y - pos1.y).powi(2)).sqrt()
}