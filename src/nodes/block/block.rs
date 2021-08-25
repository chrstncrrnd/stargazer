use macroquad::prelude::*;

use crate::resources::BlockResources;

pub enum Blocks {
    Dirt,
    Grass,
    Sand,
    Error,
}

pub enum Facing{
    Left,
    Right,
    Up,
    Down
}

pub struct Block {
    pub position: Vec2,
    pub size: f32,
    pub block_type: Blocks,
    pub facing: Facing
}

impl Default for Block {
    fn default() -> Self {
        Block {
            position: Vec2::new(0.0, 0.0),
            size: 70.,
            block_type: Blocks::Error,
            facing: Facing::Up
        }
    }
}

impl Block {
    pub fn new(position_: Vec2, block_type_: Blocks, size: usize, facing: Facing) -> Self {
        Block {
            position: position_,
            size: size as f32,
            block_type: block_type_,
            facing: facing
        }
    }

    pub fn render(&self, textures: &BlockResources) {
        draw_texture_ex(
            match self.block_type {
                Blocks::Dirt => textures.dirt,
                Blocks::Grass => textures.grass,
                Blocks::Sand => textures.sand,
                Blocks::Error => textures.error,
            },
            self.position.x,
            self.position.y,
            WHITE,
            DrawTextureParams {
                dest_size: Option::from(vec2(self.size, self.size)),
                rotation: match self.facing {
                    Facing::Left => -1.5708,
                    Facing::Right => 1.5708,
                    Facing::Up => 0.,
                    Facing::Down => 3.14159
                } as f32,
                ..Default::default()
            },
        );
    }
}