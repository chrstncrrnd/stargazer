use macroquad::prelude::*;

pub struct Planet{
    pub color: Color,
    pub radius: f32,
    pub pos_x: f32,
    pub pos_y: f32
}

impl Planet{

    pub fn render(&self){
        draw_circle(self.pos_x, self.pos_y, self.radius, self.color);
    }

}


