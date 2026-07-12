use crate::world::block::Block;
use crate::world::terrain::BlockType::*;

use bracket_noise::prelude::NoiseType::PerlinFractal;
use bracket_noise::prelude::{FastNoise, NoiseType};
use macroquad::math::Vec2;

pub const SEED: u64 = 43892;
pub const NOISE_SCALING_FACTOR: f32 = 3000.0;
pub const MAX_HEIGHT: f32 = 128.;

#[allow(dead_code)]
pub enum BlockType {
    Dirt,
    Grass,
    Ice,
    Lava,
    Leaves,
    Sand,
    Snow,
    Stone,
    Water,
    WaterDeep,
    WoodLog,
    WoodPlanks,
}

//i love traits
pub trait TerrainGenerator {
    fn get_block(&mut self, position: Vec2) -> Block;
}

#[allow(dead_code)]
// grass only terrain generator
pub struct GrassOnly;

impl TerrainGenerator for GrassOnly {
    fn get_block(&mut self, position: Vec2) -> Block {
        Block::new(position, self.get_block_type(position), 0)
    }
}

impl GrassOnly {
    fn get_block_type(&mut self, _position: Vec2) -> BlockType {
        BlockType::Grass
    }
}

#[allow(dead_code)]
// Alpha terrain idk
pub struct AlphaTerrain {
    pub noise: FastNoise,
}

#[allow(dead_code)]
impl AlphaTerrain {
    pub fn new() -> AlphaTerrain {
        let mut noise = FastNoise::seeded(SEED);
        noise.set_noise_type(PerlinFractal);
        noise.set_frequency(2.);
        AlphaTerrain { noise }
    }
}

impl TerrainGenerator for AlphaTerrain {
    fn get_block(&mut self, position: Vec2) -> Block {
        Block::new(position, self.get_block_type(position), 0)
    }
}

impl AlphaTerrain {
    fn get_block_type(&mut self, position: Vec2) -> BlockType {
        let current_noise = self.noise.get_noise(
            position.x / NOISE_SCALING_FACTOR,
            position.y / NOISE_SCALING_FACTOR,
        );
        if current_noise < 0.0 {
            BlockType::Grass
        } else {
            BlockType::Dirt
        }
    }
}

pub struct BetterTerrain {
    pub mountain_noise_cellular: FastNoise,
    pub land_or_sea_perlin: FastNoise,
    pub tree_perlin: FastNoise,
}

impl BetterTerrain {
    pub fn new() -> BetterTerrain {
        let mut mountain_noise_cellular = FastNoise::seeded(SEED);
        mountain_noise_cellular.set_noise_type(NoiseType::Cubic);
        mountain_noise_cellular.set_frequency(0.001);

        let mut land_or_sea_perlin = FastNoise::seeded(SEED);
        land_or_sea_perlin.set_noise_type(NoiseType::Perlin);
        land_or_sea_perlin.set_frequency(0.5);

        let mut tree_perlin = FastNoise::seeded(SEED);
        tree_perlin.set_noise_type(NoiseType::Perlin);
        tree_perlin.set_frequency(5.);
        BetterTerrain {
            mountain_noise_cellular,
            land_or_sea_perlin,
            tree_perlin,
        }
    }
}

impl BetterTerrain {
    fn get_block_type(&mut self, position: Vec2) -> BlockType {
        let land_or_sea = self.land_or_sea_perlin.get_noise(
            position.x / NOISE_SCALING_FACTOR,
            position.y / NOISE_SCALING_FACTOR,
        );
        if land_or_sea < -0.1 {
            //its water
            Water
        } else if land_or_sea < -0.07 {
            Sand
        } else {
            //its land
            //generate some mountains :sungalasses:
            let mountain_noise = self
                .mountain_noise_cellular
                .get_noise(position.x, position.y);
            if mountain_noise < -0.5 {
                //snowy
                Snow
            } else if mountain_noise < 0.2 {
                //its a mountain
                Stone
            } else {
                //its not a mountain
                let tree_noise = self.tree_perlin.get_noise(
                    position.x / NOISE_SCALING_FACTOR,
                    position.y / NOISE_SCALING_FACTOR,
                );
                if tree_noise < 0. {
                    //tree
                    Leaves
                } else {
                    //not a tree
                    Grass
                }
            }
        }
    }

    fn get_block_shadow(&mut self, position: Vec2) -> u8 {
        let at_pos = self
            .mountain_noise_cellular
            .get_noise(position.x, position.y);
        let below = self
            .mountain_noise_cellular
            .get_noise(position.x, position.y - 1.);
        if at_pos > below { 1 } else { 0 }
    }
}

impl TerrainGenerator for BetterTerrain {
    fn get_block(&mut self, position: Vec2) -> Block {
        Block::new(
            position,
            self.get_block_type(position),
            self.get_block_shadow(position),
        )
    }
}
