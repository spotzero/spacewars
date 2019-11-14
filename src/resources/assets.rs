use std::collections::HashMap;

use amethyst::{
    assets::{AssetStorage, Handle, Loader},
    ecs::prelude::*,
    renderer::{
        formats::texture::ImageFormat,
        sprite::{SpriteRender, SpriteSheetFormat, SpriteSheetHandle},
        SpriteSheet, Texture,
    },
    renderer::rendy::hal::image::{Anisotropic, Filter, Lod, SamplerInfo, WrapMode},
    renderer::rendy::texture::image::{ImageTextureConfig, Repr, TextureKind},
};

#[derive(Default)]
pub struct SpriteSheetManager {
    sprites: HashMap<String, Handle<SpriteSheet>>
}

impl SpriteSheetManager {

    pub fn insert(&mut self, world: &mut World, name: &str) {
        self.sprites.insert(name.to_string(), load_sprite_sheet(world, &name.to_string()));
    }

    pub fn get_handle(&self, name: &str) -> Option<&SpriteSheetHandle> {
        self.sprites.get(name)
    }

    pub fn get_render(&self, name: &str) -> Option<SpriteRender> {
      self.get_render_sprite(name, 0)
    }

    pub fn get_render_sprite(&self, name: &str, number: usize) -> Option<SpriteRender> {
        let handle = self.get_handle(name);
        match handle {
            Some(h) => Some(SpriteRender {
                sprite_sheet: h.clone(),
                sprite_number: number,
            }),
            None => None,
        }
    }
}

fn load_sprite_sheet(world: &mut World, texture: &str) -> Handle<SpriteSheet>{

        let mut sampler = SamplerInfo::new(Filter::Linear, WrapMode::Clamp);
        sampler.lod_bias = Lod::from(0.1);
        sampler.anisotropic = Anisotropic::On(100);

        let my_config = ImageTextureConfig {
            format: None,
            repr: Repr::Srgb,
            kind: TextureKind::D2,
            sampler_info: sampler,
            generate_mips: true,
            premultiply_alpha: true,
        };

        let texture_handle = {
            let loader = world.read_resource::<Loader>();
            let texture_storage = world.read_resource::<AssetStorage<Texture>>();
            loader.load(
                format!("textures/{}.png", texture),
                ImageFormat(my_config),
                (),
                &texture_storage,
            )
        };

        let loader = world.read_resource::<Loader>();
        let sprite_sheet_store = world.read_resource::<AssetStorage<SpriteSheet>>();
        loader.load(
            format!("textures/{}.ron", texture),
            SpriteSheetFormat(texture_handle), // We pass it the texture we want it to use
            (),
            &sprite_sheet_store,
        )
}
