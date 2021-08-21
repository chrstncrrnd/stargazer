use macroquad::prelude::*;

#[derive(Clone, Copy)]
pub struct Block{
    pub position: Vec2,
    pub texture: Option<Texture2D>,
    pub size: Vec2
}

impl Default for Block{
    fn default() -> Self {
        Block{
            position: Vec2::new(0.0,0.0),
            texture: None,
            size: vec2(70., 70.)
        }
    }
}

impl Block{
    pub fn render(&self){
        match self.texture{
            Some(texture) => {draw_texture_ex(
                texture,
                self.position.x,
                self.position.y,
                WHITE,
                DrawTextureParams{
                    dest_size: Option::from(vec2(self.size.x, self.size.y)),
                    ..Default::default()
                }
            ) }
            None => {draw_rectangle(self.position.x, self.position.y, 100.0, 100.0, WHITE )}
        }
    }
}