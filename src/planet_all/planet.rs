use macroquad::prelude::*;

pub struct Planet{
    pub color: Color,
    pub radius: f32,
    pub pos_x: f32,
    pub pos_y: f32
}

pub enum PlanetType{
    Sea,
    Snowy,
    Forest,
    City,
    Ideal,
    Desert,
    Swamp
}

impl Planet{

    pub fn render(&self){
        draw_circle(self.pos_x, self.pos_y, self.radius, self.color);
    }

}


