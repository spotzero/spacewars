use amethyst::{
    core::math::Vector3,
    core::timing::Time,
    core::transform::Transform,
    core::SystemDesc,
    derive::SystemDesc,
    ecs::prelude::{Join, Read, ReadStorage, System, SystemData, World, Write, WriteExpect, WriteStorage},
    ecs::{world::EntitiesRes, Entities, Entity, LazyUpdate, ReadExpect},
    input::{InputHandler, StringBindings},
    renderer::debug_drawing::DebugLinesComponent,
    renderer::palette::Srgba,
    renderer::transparent::Transparent,
};

use crate::{components::*, resources::*};

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
        ReadExpect<'s, AssetManager>,
        ReadExpect<'s, LazyUpdate>,
        WriteExpect<'s, AudioEvents>,
        Read<'s, Time>,
    );

    fn run(
        &mut self,
        (
            entities,
            transforms,
            movables,
            mut players,
            mut energies,
            input,
            asset_manager,
            lazy_update,
            mut audio_events,
            time,
        ): Self::SystemData,
    ) {
        for (entity, transform, movable, player, energy) in (
            &entities,
            &transforms,
            &movables,
            &mut players,
            &mut energies,
        )
            .join()
        {
            let fire_torpedo = input
                .action_is_down(&format!("torpedo_p{}", player.id))
                .expect("Shoot action exists");
            if fire_torpedo
                && player.last_torpedo + player.torpedo_interval < time.absolute_real_time_seconds()
                && energy.charge > player.torpedo_energy
            {
                energy.charge -= player.torpedo_energy;
                player.last_torpedo = time.absolute_real_time_seconds();
                audio_events.events.push(AudioEvent::Torpedo);
                spawn_torpedo(
                    &entity,
                    &transform,
                    &movable,
                    &lazy_update,
                    &entities,
                    &asset_manager,
                    &time,
                );
            }
        }
    }
}

#[derive(SystemDesc)]
pub struct ExplodeTorpedoSystem;

impl<'s> System<'s> for ExplodeTorpedoSystem {
    type SystemData = (
        Entities<'s>,
        ReadStorage<'s, Transform>,
        ReadStorage<'s, Movable>,
        WriteStorage<'s, Torpedo>,
        ReadExpect<'s, AssetManager>,
        ReadExpect<'s, LazyUpdate>,
        WriteExpect<'s, AudioEvents>,
        Read<'s, Time>,
    );

    fn run(
        &mut self,
        (
        entities,
        transforms,
        movables,
        mut torpedos,
        asset_manager,
        lazy_update,
        mut audio_events,
        time
    ): Self::SystemData,
    ) {
        for (entity, transform, movable, torpedo) in
            (&entities, &transforms, &movables, &mut torpedos).join()
        {
            if (torpedo.fired + torpedo.life) <= time.absolute_real_time_seconds() {
                audio_events.events.push(AudioEvent::ExplosionTorpedo);
                explode_torpedo(
                    &transform,
                    &movable,
                    &lazy_update,
                    &entities,
                    &asset_manager,
                    &time,
                    entity,
                );
            }
        }
    }
}

#[derive(SystemDesc)]
pub struct TorpedoCollisionResponseSystem;

impl<'s> System<'s> for TorpedoCollisionResponseSystem {
    type SystemData = (
        Entities<'s>,
        ReadStorage<'s, Transform>,
        ReadStorage<'s, Movable>,
        ReadStorage<'s, Torpedo>,
        ReadExpect<'s, AssetManager>,
        ReadExpect<'s, LazyUpdate>,
        WriteExpect<'s, CollisionEvents>,
        WriteExpect<'s, AudioEvents>,
        Read<'s, Time>,
    );

    fn run(
        &mut self,
        (
            entities,
            transforms,
            movables,
            torpedos,
            asset_manager,
            lazy_update,
            mut collision_events,
            mut audio_events,
            time,
        ): Self::SystemData,
    ) {
        for (entity, transform, movable, torpedo) in
            (&entities, &transforms, &movables, &torpedos).join()
        {
            for i in 0..collision_events.torpedo_collisions.len() {
                if collision_events.torpedo_collisions[i].torpedo == entity.id()
                    && torpedo.player != collision_events.torpedo_collisions[i].collided
                {
                    //let mut result_move = movable.clone();
                    //result_move.velocity = result_move.velocity - (collision_events.torpedo_collisions[i].direction * 100.0);
                    audio_events.events.push(AudioEvent::ExplosionTorpedo);
                    explode_torpedo(
                        &transform,
                        &movable,
                        &lazy_update,
                        &entities,
                        &asset_manager,
                        &time,
                        entity,
                    );
                }
            }
        }
        collision_events.torpedo_collisions.clear();
    }
}

fn explode_torpedo(
    transform: &Transform,
    movable: &Movable,
    lazy_update: &LazyUpdate,
    entities: &Read<EntitiesRes>,
    asset_manager: &AssetManager,
    time: &Time,
    entity: Entity,
) {
    generate_explosion(
        &transform,
        &movable,
        50.0,
        0.4,
        200.,
        &entities,
        &asset_manager,
        &lazy_update,
        &time,
        Explosion {
            vel: 200.0,
            dsp: 200.0,
        },
        0,
    );
    let _ = entities.delete(entity);
}

fn spawn_torpedo(
    entity: &Entity,
    transform: &Transform,
    movable: &Movable,
    lazy_update: &LazyUpdate,
    entities: &Read<EntitiesRes>,
    asset_manager: &AssetManager,
    time: &Time,
) {
    let mut thrustvector = Vector3::new(0.0, 40.0, 0.0);
    thrustvector = transform.rotation().transform_vector(&thrustvector);
    let mut pos = transform.clone();
    pos.set_scale(Vector3::new(0.05, 0.05, 1.0));
    pos.move_forward(0.1);
    let part: Entity = entities.create();
    lazy_update.insert(
        part,
        asset_manager
            .get_render("weapons/missle-001")
            .unwrap(),
    );
    lazy_update.insert(part, pos);
    lazy_update.insert(part, ParticleCom);
    lazy_update.insert(part, Transparent);
    lazy_update.insert(
        part,
        Collidable {
            kind: collidable_types::TORPEDO,
            radius: 20.0,
            ignore: Some(*entity),
        },
    );
    lazy_update.insert(part, DebugLinesComponent::with_capacity(16));
    lazy_update.insert(
        part,
        Ship {
            hull: 5.,
            max_hull: 5.,
            shield: 0.0,
            max_shield: 0.0,
            thrust: 4000.0,
            torque: 0.0,
            thrust_failure: false,
            torque_failure: false,
            applying_thrust: 1.0,
            applying_torque: 0.0,
            shield_entity: None,
        },
    );
    lazy_update.insert(
        part,
        Torpedo {
            fired: time.absolute_real_time_seconds(),
            life: 2.0,
            player: entity.id(),
        },
    );
    lazy_update.insert(
        part,
        Energy {
            charge: 50.0,
            recharge_rate: 0.0,
            max_charge: 50.0,
        },
    );
    lazy_update.insert(
        part,
        Movable {
            velocity: movable.velocity + thrustvector,
            angular_velocity: movable.angular_velocity / 2.,
            mass: 10.0,
            apply_physics: true,
        },
    );
    lazy_update.insert(
        part,
        ShipEngines {
            engines: [Engine {
                location: Vector3::new(0.0, -14.0, 0.05),
                direction: 1,
                rotate: 0,
                tint: Srgba::new(0.9, 0.6, 0.0, 1.0),
                last_emit: 0.0,
                emit_rate: 0.02,
            }]
            .to_vec(),
        },
    );
}
