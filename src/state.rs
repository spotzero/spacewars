use amethyst::{
    assets::{AssetStorage, Handle, Loader},
    core::math::Vector3,
    core::transform::Transform,
    input::{is_close_requested, is_key_down, VirtualKeyCode},
    prelude::*,
    renderer::{Camera, ImageFormat, SpriteRender, SpriteSheet, SpriteSheetFormat, Texture, Transparent},
    renderer::palette::Srgba,
    renderer::rendy::hal::image::{Anisotropic, Filter, Lod, SamplerInfo, WrapMode},
    renderer::rendy::texture::image::{ImageTextureConfig, Repr, TextureKind},
//    window::ScreenDimensions,
};

use crate::{ARENA_HEIGHT, ARENA_WIDTH};
use crate::components::*;

pub struct SpacewarsState;

impl SimpleState for SpacewarsState {
    // On start will run when this state is initialized. For more
    // state lifecycle hooks, see:
    // https://book.amethyst.rs/stable/concepts/state.html#life-cycle
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        let world = data.world;

        let bg_render = SpriteRender {
            sprite_sheet: load_sprite_sheet(world, "backgrounds/background-2").clone(),
            sprite_number: 0, // paddle is the first sprite in the sprite_sheet
        };
        let mut bg_transform = Transform::default();
        bg_transform.set_translation_xyz(ARENA_WIDTH/2.0, ARENA_HEIGHT/2.0, -10.0);

        world
            .create_entity()
            .with(bg_render)
            .with(bg_transform)
            .build();

        let gravitywell_render = SpriteRender {
            sprite_sheet: load_sprite_sheet(world, "backgrounds/gravity-well").clone(),
            sprite_number: 0, // paddle is the first sprite in the sprite_sheet
        };
        let mut gravitywell_transform = Transform::default();
        gravitywell_transform.set_translation_xyz(ARENA_WIDTH/2.0, ARENA_HEIGHT/2.0, -9.0);
        gravitywell_transform.set_scale(Vector3::new(0.5,0.5,1.0));

        world
            .create_entity()
            .with(gravitywell_render)
            .with(gravitywell_transform)
            .with(Transparent)
            .build();

        let ship_render = SpriteRender {
            sprite_sheet: load_sprite_sheet(world, "ships/ship-001").clone(),
            //sprite_sheet: load_sprite_sheet(world, "particles/particle0").clone(),
            sprite_number: 0, // paddle is the first sprite in the sprite_sheet
        };
        let mut ship_transform = Transform::default();
        ship_transform.set_translation_xyz(ARENA_WIDTH/4.0, ARENA_HEIGHT/2.0, 0.0);
        ship_transform.set_scale(Vector3::new(0.1,0.1,1.0));

        world
            .create_entity()
            .with(ship_render.clone())
            .with(ship_transform)
            .with(Transparent)
            .with(Movable{
                velocity: Vector3::new(0.0,120.0,0.0),
                angular_velocity: 0.0,
                mass: 150.0,
            })
            .with(Ship {
                hull: 50.0,
                shield: 75.0,
                thrust: 50000.0,
                torque: 600.0,
                applying_thrust: 0.0,
                applying_torque: 0.0,
            })/*
            .with(Engine {
                location: Vector3::new(0.0, 2.0, -1.0),
                direction: true,
                tint: Srgba::new(1.0, 0.1, 0.1, 1.0),
            })*/
            .with(Player {
                controllable: true,
                id: 1,
                last_torpedo: 0.0,
                last_missle: 0.0,
                last_hyperspace: 0.0,
            })
            .build();

        let mut ship_transform = Transform::default();
        ship_transform.set_translation_xyz(3.0*(ARENA_WIDTH/4.0), ARENA_HEIGHT/2.0, 0.0);
        ship_transform.set_scale(Vector3::new(0.1,0.1,1.0));

        world
            .create_entity()
            .with(ship_render.clone())
            .with(ship_transform)
            .with(Transparent)
            .with(Movable{
                velocity: Vector3::new(0.0,-120.0,0.0),
                angular_velocity: 0.0,
                mass: 150.0,
            })
            .with(Ship {
                hull: 50.0,
                shield: 75.0,
                thrust: 50000.0,
                torque: 600.0,
                applying_thrust: 0.0,
                applying_torque: 0.0,
            })/*
            .with(Engine {
                location: Vector3::new(0.0, 2.0, -1.0),
                direction: true,
                tint: Srgba::new(0.1, 0.1, 0.1, 1.0),
            })*/
            .with(Player {
                controllable: true,
                id: 2,
                last_torpedo: 0.0,
                last_missle: 0.0,
                last_hyperspace: 0.0,
            })
            .build();

        // Place the camera
        initialise_camera(world);
    }

    fn handle_event(
        &mut self,
        mut _data: StateData<'_, GameData<'_, '_>>,
        event: StateEvent,
    ) -> SimpleTrans {
        if let StateEvent::Window(event) = &event {
            // Check if the window should be closed
            if is_close_requested(&event) || is_key_down(&event, VirtualKeyCode::Escape) {
                return Trans::Quit;
            }
        }
        // Keep going
        Trans::None
    }
}

/// Initialise the camera.
fn initialise_camera(world: &mut World) {
    // Setup camera in a way that our screen covers whole arena and (0, 0) is in the bottom left.
    let mut transform = Transform::default();
    transform.set_translation_xyz(ARENA_WIDTH * 0.5, ARENA_HEIGHT * 0.5, 1.0);

    world
        .create_entity()
        .with(Camera::standard_2d(ARENA_WIDTH, ARENA_HEIGHT))
        .with(transform)
        .build();
}

fn load_sprite_sheet(world: &mut World, texture: &str) -> Handle<SpriteSheet> {

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
