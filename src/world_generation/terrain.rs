use bracket_noise::prelude::FastNoise;
use bracket_noise::prelude::NoiseType::PerlinFractal;
use macroquad::math::Vec2;
use std::thread::current;

#[allow(dead_code)]
pub enum BlockType {
    Dirt,
    Error,
    Grass,
    Sand,
    Mud,
    Water,
}

pub trait TerrainGenerator {
    fn init(&mut self);
    fn get_block(&self, position: Vec2) -> BlockType;
}
//i love traits

pub struct GrassOnly;
impl TerrainGenerator for GrassOnly {
    fn init(&mut self) {}

    fn get_block(&self, position: Vec2) -> BlockType {
        BlockType::Grass
    }
}

pub struct AlphaTerrain {
    pub noise: Option<FastNoise>,
}
impl TerrainGenerator for AlphaTerrain {
    fn init(&mut self) {
        let mut noise = FastNoise::seeded(crate::world_generation::seed::get_seed());
        noise.set_noise_type(PerlinFractal);
        noise.set_frequency(2.);
        self.noise = Option::from(noise);
    }
    fn get_block(&self, position: Vec2) -> BlockType {
        let current_noise = self
            .noise
            .as_ref()
            .unwrap()
            .get_noise(position.x / 3000., position.y / 3000.);
        if current_noise < 0.0 {
            return BlockType::Grass;
        } else {
            return BlockType::Dirt;
        }
    }
}

pub struct BetterTerrain{
    pub noise: Option<FastNoise>
}

impl TerrainGenerator for BetterTerrain{
    fn init(&mut self) {
        let mut noise = FastNoise::seeded(crate::world_generation::seed::get_seed());
        noise.set_noise_type(PerlinFractal);
        noise.set_frequency(2.);
        self.noise = Option::from(noise);
    }
    fn get_block(&self, position: Vec2) -> BlockType {
        let current_noise = self
            .noise
            .as_ref()
            .unwrap()
            .get_noise(position.x / 3000., position.y / 3000.);
        if current_noise < -0.1{
            return BlockType::Water;
        }
        if current_noise < -0.05 {
            return BlockType::Sand;
        } else if current_noise < 0.1{
            return BlockType::Grass;
        } else {
            return BlockType::Dirt;
        }
    }
}
