use std::collections::HashMap;

use amethyst::{
    assets::{AssetStorage, Handle, Loader, ProgressCounter},
    audio::{output::Output, Source, SourceHandle, WavFormat},
    ecs::prelude::*,
    renderer::rendy::hal::image::{Anisotropic, Filter, Lod, SamplerInfo, WrapMode},
    renderer::rendy::texture::image::{ImageTextureConfig, Repr, TextureKind},
    renderer::{
        formats::texture::ImageFormat,
        sprite::{SpriteRender, SpriteSheetFormat, SpriteSheetHandle},
        SpriteSheet, Texture,
    },
    ui::{FontAsset, TtfFormat},
};

#[derive(Default)]
pub struct AssetManager {
    pub progress: ProgressCounter,
    sprites: HashMap<String, Handle<SpriteSheet>>,
    sounds: HashMap<String, SourceHandle>,
    font: Option<Handle<FontAsset>>,
}

#[derive(Debug)]
pub enum AssetKind {
    Sprite,
    Sound,
    Font,
}

impl AssetManager {
    pub fn insert(&mut self, world: &World, name: &str, kind: AssetKind) {
        match kind {
            AssetKind::Sprite => {
                self.sprites.insert(
                    name.to_string(),
                    load_sprite_sheet(world, &name.to_string(), &mut self.progress),
                );
            }
            AssetKind::Sound => {
                self.sounds.insert(
                    name.to_string(),
                    world.read_resource::<Loader>().load(
                        format!("sounds/{}.wav", name),
                        WavFormat,
                        &mut self.progress,
                        &world.read_resource(),
                    ),
                );
            }
            AssetKind::Font => {
                self.font = Some(world.read_resource::<Loader>().load(
                    name,
                    TtfFormat,
                    &mut self.progress,
                    &world.read_resource(),
                ));
            }
        }
    }

    pub fn get_wav(&self, name: &str) -> Option<&SourceHandle> {
        self.sounds.get(name)
    }

    pub fn play_wav(&self, name: &str, storage: &AssetStorage<Source>, output: &Output) {
        if let Some(sound) = storage.get(self.get_wav(name).expect("Invalid sound loaded")) {
            output.play_once(sound, 1.0);
        }
    }

    pub fn font(&self) -> Option<Handle<FontAsset>> {
        if self.font.is_some() {
            self.font.clone()
        } else {
            None
        }
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

fn load_sprite_sheet(
    world: &World,
    texture: &str,
    progress: &mut ProgressCounter,
) -> Handle<SpriteSheet> {
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
            progress,
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
