use amethyst::{
    assets::{AssetStorage, Handle, Loader},
    core::math::Vector2,
    ecs::World,
    renderer::{SpriteSheet, Texture, ImageFormat, SpriteSheetFormat, SpriteRender},
    prelude::*
};

pub struct BunnyResource {
    pub sprite_render: SpriteRender,
    pub sprite_size: Vector2<f32>,
    pub bounds: Vector2<f32>,
}

impl BunnyResource {
    pub fn initialize(world: &mut World) {

        let texture_handle = {
            let loader = world.read_resource::<Loader>();
            let texture_storage = world.read_resource::<AssetStorage<Texture>>();
            loader.load(
                "icon.png",
                ImageFormat::default(),
                (),
                &texture_storage,
            )
        };

        /*
        let texture_id = 0;

        {
            let mut material_texture_set = world.write_resource::<MaterialTextureSet>();
            material_texture_set.insert(texture_id, texture_handle);
        }*/

        let sprite_sheet_handle = {
            let loader = world.read_resource::<Loader>();
            let sprite_sheet_store = world.read_resource::<AssetStorage<SpriteSheet>>();
            loader.load(
                "bunny_spritesheet.ron", // Here we load the associated ron file
                SpriteSheetFormat(texture_handle),
                (),
                &sprite_sheet_store,
            )
        };

        let bunny_resources = BunnyResource {
            sprite_render: SpriteRender::new(sprite_sheet_handle, 0),
            bounds: Vector2::new(500.0, 500.0),
            sprite_size: Vector2::new(64.0, 64.0),
        };

        world.insert(bunny_resources);
    }
}