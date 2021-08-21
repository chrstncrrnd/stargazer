use macroquad::prelude::*;

pub struct BlockResources {
    pub dirt: Texture2D,
    pub grass: Texture2D,
    pub sand: Texture2D,
    pub error: Texture2D,
}

impl BlockResources {
    async fn new() -> Result<BlockResources, FileError> {
        let dirt = load_texture("assets/blocks/dirt.png").await?;
        dirt.set_filter(FilterMode::Nearest);

        let grass = load_texture("assets/blocks/grass.png").await?;
        grass.set_filter(FilterMode::Nearest);

        let sand = load_texture("assets/blocks/sand.png").await?;
        sand.set_filter(FilterMode::Nearest);

        let error = load_texture("assets/blocks/error.png").await?;
        error.set_filter(FilterMode::Nearest);

        Ok(BlockResources {
            dirt,
            grass,
            sand,
            error,
        })
    }
}

pub struct Resources {
    pub player_texture: Texture2D,
    pub font: Font,
    pub block_resources: BlockResources,
}

impl Resources {
    pub async fn new() -> Result<Resources, FileError> {
        let player_texture = load_texture("assets/player_texture_default.png").await?;
        player_texture.set_filter(FilterMode::Nearest);

        let block_resources = BlockResources::new().await?;

        let font = load_ttf_font("assets/Font.ttf").await.unwrap();

        Ok(Resources {
            player_texture,
            font,
            block_resources,
        })
    }
}
