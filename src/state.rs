use amethyst::{
    core::math::Vector3,
    core::transform::Transform,
    input::{is_close_requested, is_key_down, VirtualKeyCode},
    prelude::*,
    renderer::{Camera, Transparent},
    renderer::palette::Srgba,
//    window::ScreenDimensions,
};

use crate::{ARENA_HEIGHT, ARENA_WIDTH};
use crate::components::*;
use crate::resources::*;

pub struct SpacewarsState;

impl SimpleState for SpacewarsState {

    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        let mut world = data.world;
        let mut sprite_sheet_manager = SpriteSheetManager::default();
        sprite_sheet_manager.insert(&mut world, "backgrounds/background-2");
        sprite_sheet_manager.insert(&mut world, "backgrounds/gravity-well");
        sprite_sheet_manager.insert(&mut world, "ships/ship-001");
        sprite_sheet_manager.insert(&mut world, "particles/particle0");
        sprite_sheet_manager.insert(&mut world, "particles/debris");
        sprite_sheet_manager.insert(&mut world, "weapons/missle-001");

        let mut bg_transform = Transform::default();
        bg_transform.set_translation_xyz(ARENA_WIDTH/2.0, ARENA_HEIGHT/2.0, -10.0);

        world
            .create_entity()
            .with(sprite_sheet_manager.get_render("backgrounds/background-2").unwrap())
            .with(bg_transform)
            .build();

        let mut gravitywell_transform = Transform::default();
        gravitywell_transform.set_translation_xyz(ARENA_WIDTH/2.0, ARENA_HEIGHT/2.0, -9.0);
        gravitywell_transform.set_scale(Vector3::new(0.5,0.5,1.0));

        world
            .create_entity()
            .with(sprite_sheet_manager.get_render("backgrounds/gravity-well").unwrap())
            .with(gravitywell_transform)
            .with(Transparent)
            .build();

        let mut ship_transform = Transform::default();
        ship_transform.set_translation_xyz(ARENA_WIDTH/4.0, ARENA_HEIGHT/2.0, 0.0);
        ship_transform.set_scale(Vector3::new(0.1,0.1,1.0));

        world
            .create_entity()
            .with(sprite_sheet_manager.get_render("ships/ship-001").unwrap())
            .with(ship_transform)
            .with(Transparent)
            .with(Collidable {
                radius: 10.0,
            })
            .with(Energy {
                charge: 100.0,
                recharge_rate: 5.0,
                max_charge: 100.0,
            })
            .with(Movable{
                velocity: Vector3::new(0.0,120.0,0.0),
                angular_velocity: 0.6,
                mass: 150.0,
            })
            .with(Ship {
                hull: 50.0,
                shield: 75.0,
                thrust: 50000.0,
                torque: 600.0,
                thrust_failure: false,
                torque_failure: false,
                applying_thrust: 0.0,
                applying_torque: 0.0,
            })
            .with(ShipEngines {
                engines: [
                    Engine {
                        location: Vector3::new(13.0, -18.0, 0.1),
                        direction: 1,
                        rotate: -1,
                        tint: Srgba::new(0.6, 0.1, 0.1, 1.0),
                        last_emit: 0.0,
                        emit_rate: 0.02,
                    },
                    Engine {
                        location: Vector3::new(-13.0, -18.0, 0.1),
                        direction: 1,
                        rotate: 1,
                        tint: Srgba::new(0.6, 0.1, 0.1, 1.0),
                        last_emit: 0.0,
                        emit_rate: 0.02,
                    },
                    Engine {
                        location: Vector3::new(13.0, 18.0, 0.1),
                        direction: -1,
                        rotate: 1,
                        tint: Srgba::new(0.6, 0.1, 0.1, 1.0),
                        last_emit: 0.0,
                        emit_rate: 0.02,
                    },
                    Engine {
                        location: Vector3::new(-13.0, 18.0, 0.1),
                        direction: -1,
                        rotate: -1,
                        tint: Srgba::new(0.6, 0.1, 0.1, 1.0),
                        last_emit: 0.0,
                        emit_rate: 0.02,
                    },
                ].to_vec(),
            })
            .with(Player {
                controllable: true,
                id: 1,
                last_torpedo: 0.0,
                torpedo_interval: 1.5,
                torpedo_energy: 10.0,
                last_hyperspace: 0.0,
                hyperspace_interval: 5.0,
            })
            .build();

        let mut ship_transform = Transform::default();
        ship_transform.set_translation_xyz(3.0*(ARENA_WIDTH/4.0), ARENA_HEIGHT/2.0, 0.0);
        ship_transform.set_rotation_2d(135.0);
        ship_transform.set_scale(Vector3::new(0.1,0.1,1.0));

        world
            .create_entity()
            .with(sprite_sheet_manager.get_render("ships/ship-001").unwrap())
            .with(ship_transform)
            .with(Transparent)
            .with(Collidable {
                radius: 10.0,
            })
            .with(Energy {
                charge: 100.0,
                recharge_rate: 5.0,
                max_charge: 100.0,
            })
            .with(Movable{
                velocity: Vector3::new(0.0,-120.0,0.0),
                angular_velocity: 0.6,
                mass: 150.0,
            })
            .with(Ship {
                hull: 50.0,
                shield: 75.0,
                thrust: 50000.0,
                torque: 600.0,
                thrust_failure: false,
                torque_failure: false,
                applying_thrust: 0.0,
                applying_torque: 0.0,
            })
            .with(Player {
                controllable: true,
                id: 2,
                last_torpedo: 0.0,
                torpedo_interval: 1.5,
                torpedo_energy: 10.0,
                last_hyperspace: 0.0,
                hyperspace_interval: 5.0,
            })
            .build();

        // Place the camera
        initialise_camera(world);
        world.insert(sprite_sheet_manager);
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
