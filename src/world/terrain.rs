use crate::world::terrain::BlockType::{
    Dirt, Grass, Leaves, Sand, Snow, Stone, Water, WaterDeep,
};
use bracket_noise::prelude::NoiseType::PerlinFractal;
use bracket_noise::prelude::{FastNoise, NoiseType};
use macroquad::math::Vec2;
use std::thread::current;

const SEED: u64 = 43892;

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
    fn get_block(&self, position: Vec2) -> BlockType;
}

// grass only terrain generator
pub struct GrassOnly;

impl TerrainGenerator for GrassOnly {
    fn get_block(&self, position: Vec2) -> BlockType {
        BlockType::Grass
    }
}

// Alpha terrain idk
pub struct AlphaTerrain {
    pub noise: FastNoise,
}

impl AlphaTerrain {
    pub fn new() -> AlphaTerrain {
        let mut noise = FastNoise::seeded(SEED);
        noise.set_noise_type(PerlinFractal);
        noise.set_frequency(2.);
        AlphaTerrain { noise }
    }
}

impl TerrainGenerator for AlphaTerrain {
    fn get_block(&self, position: Vec2) -> BlockType {
        let current_noise = self.noise.get_noise(position.x / 3000., position.y / 3000.);
        if current_noise < 0.0 {
            return BlockType::Grass;
        } else {
            return BlockType::Dirt;
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

impl TerrainGenerator for BetterTerrain {
    fn get_block(&self, position: Vec2) -> BlockType {
        let land_or_sea = self
            .land_or_sea_perlin
            .get_noise(position.x / 3000., position.y / 3000.);
        if land_or_sea < -0.1 {
            //its water
            return Water;
        } else if land_or_sea < -0.07 {
            return Sand;
        } else {
            //its land
            //generate some mountains :sungalasses:
            let mountain_noise = self
                .mountain_noise_cellular
                .get_noise(position.x, position.y);
            if mountain_noise < -0.5 {
                //snowy
                return Snow;
            } else if mountain_noise < 0.2 {
                //its a mountain
                return Stone;
            } else {
                //its not a mountain
                let tree_noise = self
                    .tree_perlin
                    .get_noise(position.x / 3000., position.y / 3000.);
                if tree_noise < 0. {
                    //tree
                    return Leaves;
                } else {
                    //not a tree
                    return Grass;
                }
            }
        }
        Dirt
    }
}
