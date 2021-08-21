use macroquad::prelude::*;

pub enum BlockTypes {
    Dirt,
    Grass,
    Sand,
    Error,
}

pub struct Block {
    pub position: Vec2,
    pub size: Vec2,
    pub block_type: BlockTypes,
    pub texture: Option<Texture2D>,
}

impl Default for Block {
    fn default() -> Self {
        Block {
            position: Vec2::new(0.0, 0.0),
            size: vec2(70., 70.),
            block_type: BlockTypes::Error,
            texture: Option::from(Texture2D::empty()),
        }
    }
}

impl Block {
    pub fn render(&self) {
        match self.texture {
            Some(texture) => draw_texture_ex(
                texture,
                self.position.x,
                self.position.y,
                WHITE,
                DrawTextureParams {
                    dest_size: Option::from(vec2(self.size.x, self.size.y)),
                    ..Default::default()
                },
            ),
            None => draw_rectangle(self.position.x, self.position.y, 100.0, 100.0, WHITE),
        }
    }
}
