use macroquad::prelude::*;
use crate::planet_all::planet::PlanetType;

enum BlockTypes{
    Path,
    Grass,
    Dirt,
    Sand
}

struct Block{
    position: Vec2,
    block_type: BlockTypes,
    planet_type: PlanetType
}