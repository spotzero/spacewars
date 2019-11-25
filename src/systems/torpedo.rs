use amethyst::{
    core::timing::Time,
    core::transform::Transform,
    core::SystemDesc,
    core::math::Vector3,
    derive::SystemDesc,
    renderer::palette::Srgba,
    ecs::prelude::{Join, Read, ReadStorage, System, SystemData, World, WriteStorage},
    ecs::{Entities, Entity, LazyUpdate, ReadExpect, world::EntitiesRes},
    input::{InputHandler, StringBindings},
    renderer::{transparent::Transparent},
};

use crate::{
    components::*,
    resources::*,
};

#[derive(SystemDesc)]
pub struct FireTorpedoSystem;

impl<'s> System<'s> for FireTorpedoSystem {
    type SystemData = (
        Entities<'s>,
        ReadStorage<'s, Transform>,
        ReadStorage<'s, Movable>,
        WriteStorage<'s, Player>,
        WriteStorage<'s, Energy>,
        Read<'s, InputHandler<StringBindings>>,
        ReadExpect<'s, SpriteSheetManager>,
        ReadExpect<'s, LazyUpdate>,
        Read<'s, Time>,
    );

    fn run(&mut self, (
        entities,
        transforms,
        movables,
        mut players,
        mut energies,
        input,
        sprite_sheet_manager,
        lazy_update,
        time
    ): Self::SystemData) {
        for (transform, movable, player, energy) in (&transforms, &movables, &mut players, &mut energies).join() {
            let fire_torpedo = input.action_is_down(&format!("torpedo_p{}", player.id)).expect("Shoot action exists");
            if
                fire_torpedo
                && player.last_torpedo + player.torpedo_interval < time.absolute_real_time_seconds()
                && energy.charge > player.torpedo_energy
            {
                energy.charge -= player.torpedo_energy;
                player.last_torpedo = time.absolute_real_time_seconds();
                spawn_torpedo(&transform, &movable, &lazy_update, &entities, &sprite_sheet_manager, &time);
                generate_explosion(
                    &transform,
                    &movable,
                    50.0,
                    &entities,
                    &sprite_sheet_manager,
                    &lazy_update,
                    &time,
                );
            }

        }
    }
}

fn spawn_torpedo(
    transform: &Transform,
    movable: &Movable,
    lazy_update: &LazyUpdate,
    entities: &Read<EntitiesRes>,
    sprite_sheet_manager: &SpriteSheetManager,
    time: &Time
) {

    let mut thrustvector = Vector3::new(0.0, 20.0,0.0);
    thrustvector = transform.rotation().transform_vector(&thrustvector);
    let mut pos = transform.clone();
    pos.set_scale(Vector3::new(0.05,0.05,1.0));
    pos.move_forward(0.1);
    let part: Entity = entities.create();
    lazy_update.insert(part, sprite_sheet_manager.get_render("weapons/missle-001").unwrap());
    lazy_update.insert(part, pos);
    lazy_update.insert(part, ParticleCom);
    lazy_update.insert(part, Transparent);
    lazy_update.insert(part, Ship {
        hull: 5.0,
        shield: 0.0,
        thrust: 2000.0,
        torque: 0.0,
        thrust_failure: false,
        torque_failure: false,
        applying_thrust: 1.0,
        applying_torque: 0.0,
    });
    lazy_update.insert(part, Collidable {kind: CollidableKind::Torpedo, radius: 5.0});
    lazy_update.insert(part, Lifetime {
        start: time.absolute_real_time_seconds(),
        life: 5.0,
    });
    lazy_update.insert(part, Energy {
        charge: 50.0,
        recharge_rate: 0.0,
        max_charge: 50.0,
    });
    lazy_update.insert(part, Movable {
        velocity: movable.velocity + thrustvector,
        angular_velocity: 0.0,
        mass: 10.0,
    });
    lazy_update.insert(part, ShipEngines {
        engines: [
            Engine {
                location: Vector3::new(0.0, -14.0, 0.05),
                direction: 1,
                rotate: 0,
                tint: Srgba::new(0.9, 0.6, 0.0, 1.0),
                last_emit: 0.0,
                emit_rate: 0.02,
            },
        ].to_vec()
    });

}
