use amethyst::{
    core::transform::Transform,
    core::math::Vector3,
    ecs::prelude::Read,
    ecs::{Entity, LazyUpdate, world::EntitiesRes},
    renderer::{transparent::Transparent, resources::Tint},
};

use crate::components::{Lifetime, Movable, ParticleCom};
use crate::resources::{SpriteSheetManager};

pub fn spawn_player(
    id: u8,
    lazy_update: &LazyUpdate,
    entities: &Read<EntitiesRes>,
    sprite_sheet_manager: &SpriteSheetManager,
) {
    let mut transform = Transform::default();
    let mut movable = Movable {
        velocity: Vector3::new(0.0,120.0,0.0),
        angular_velocity: 0.6,
        mass: 150.0,
    };
    let mut tint = Srgba::new(0.6, 0.1, 0.1, 1.0);
    transform.set_scale(Vector3::new(0.1,0.1,1.0));

    if id == 1 {
        transform.set_translation_xyz(ARENA_WIDTH/4.0, ARENA_HEIGHT/2.0, 0.0);
    } else {
        transform.set_translation_xyz(3.0*(ARENA_WIDTH/4.0), ARENA_HEIGHT/2.0, 0.0);
        transform.set_rotation_2d(135.0);
        movable.velocity.y = -120.0;
        tint = tint: Srgba::new(0.1, 0.1, 0.6, 1.0);
    }

    let player: Entity = entities.create();
    lazy_update.insert(player, sprite_sheet_manager.get_render("ships/ship-001").unwrap());
    lazy_update.insert(player, transform);
    lazy_update.insert(part, Transparent);
    lazy_update.insert(part, Energy {
        charge: 100.0,
        recharge_rate: 5.0,
        max_charge: 100.0,
    });
    lazy_update.insert(player, movable);
    lazy_update.insert(player, Ship {
        hull: 50.0,
        shield: 75.0,
        thrust: 50000.0,
        torque: 600.0,
        thrust_failure: false,
        torque_failure: false,
        applying_thrust: 0.0,
        applying_torque: 0.0,
    });
    lazy_update.insert(player, Collidable{
        kind: collidable_types::PLAYER,
        radius: 25.0,
    });
    lazy_update.insert(player, ShipEngines {
        engines: [
            Engine {
                location: Vector3::new(13.0, -18.0, 0.1),
                direction: 1,
                rotate: -1,
                tint: tint.clone(),
                last_emit: 0.0,
                emit_rate: 0.02,
            },
            Engine {
                location: Vector3::new(-13.0, -18.0, 0.1),
                direction: 1,
                rotate: 1,
                tint: tint.clone(),
                last_emit: 0.0,
                emit_rate: 0.02,
            },
            Engine {
                location: Vector3::new(13.0, 18.0, 0.1),
                direction: -1,
                rotate: 1,
                tint: tint.clone(),
                last_emit: 0.0,
                emit_rate: 0.02,
            },
            Engine {
                location: Vector3::new(-13.0, 18.0, 0.1),
                direction: -1,
                rotate: -1,
                tint: tint.clone(),
                last_emit: 0.0,
                emit_rate: 0.02,
            },
        ].to_vec(),
    });
    lazy_update.insert(player, Player {
        controllable: true,
        id: id,
        last_torpedo: 0.0,
        torpedo_interval: 1.5,
        torpedo_energy: 10.0,
        last_hyperspace: 0.0,
        hyperspace_interval: 5.0,
    });
}
