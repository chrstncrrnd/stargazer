use macroquad::math::{Vec2, vec2};

pub fn i32_vec2(x: i32, y: i32) -> Vec2{
    vec2(x as f32, y as f32)
}

pub fn vec2_i32_pair(vector: Vec2) -> (i32, i32){
    (vector.x as i32, vector.y as i32)
}
