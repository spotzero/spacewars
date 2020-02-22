use amethyst::{
    core::math::UnitQuaternion,
    core::math::Vector3,
    core::timing::Time,
    core::transform::Transform,
    derive::SystemDesc,
    ecs::prelude::{Join, Read, ReadExpect, ReadStorage, System, SystemData, WriteStorage},
    ecs::{world::EntitiesRes, Entities, LazyUpdate},
    renderer::resources::Tint,
};

use rand::Rng;

use crate::{components::*, resources::*};

// ParticleSystem controls the lifetime and fade of emitted particles.
#[derive(SystemDesc)]
pub struct ParticleSystem;

// EngineParticleSystem creates particles when an engine is thrusting, in the reverse direction of thrust.
#[derive(SystemDesc)]
pub struct EngineParticleSystem;

impl<'s> System<'s> for ParticleSystem {
    type SystemData = (
        Entities<'s>,
        ReadStorage<'s, ParticleCom>,
        ReadStorage<'s, Lifetime>,
        Read<'s, Time>,
        ReadExpect<'s, Game>,
    );

    fn run(&mut self, (entities, _particles, lifetimes, time, game): Self::SystemData) {
        if !game.is_playing() {
            return;
        }
        for (entity, lifetime) in (&entities, &lifetimes).join() {
            if lifetime.start + lifetime.life < time.absolute_time_seconds() {
                let _ = entities.delete(entity);
            }
        }
    }
}

impl<'s> System<'s> for EngineParticleSystem {
    type SystemData = (
        Entities<'s>,
        ReadStorage<'s, Ship>,
        ReadStorage<'s, Transform>,
        ReadStorage<'s, Movable>,
        WriteStorage<'s, ShipEngines>,
        ReadExpect<'s, AssetManager>,
        ReadExpect<'s, LazyUpdate>,
        Read<'s, Time>,
        ReadExpect<'s, Game>,
    );

    fn run(
        &mut self,
        (
            entities,
            ships,
            transforms,
            movables,
            mut ship_engines,
            asset_manager,
            lazy_update,
            time,
            game,
        ): Self::SystemData,
    ) {
        if !game.is_playing() {
            return;
        }
        for (ship, transform, mover, ship_engine) in
            (&ships, &transforms, &movables, &mut ship_engines).join()
        {
            if ship.applying_thrust != 0.0 || ship.applying_torque != 0.0 {
                for i in 0..ship_engine.engines.len() {
                    if check_engine(&ship_engine.engines[i], &ship, &time) {
                        for _j in 0..3 {
                            emit_particle_for(
                                &mut ship_engine.engines[i],
                                &time,
                                &lazy_update,
                                transform,
                                mover,
                                &entities,
                                &asset_manager,
                            );
                        }
                    }
                }
            }
        }
    }
}

fn check_engine(engine: &Engine, ship: &Ship, time: &Time) -> bool {
    if engine.last_emit + engine.emit_rate < time.absolute_time_seconds()
        && ((ship.applying_thrust > 0.0 && engine.direction > 0 && !ship.thrust_failure)
            || (ship.applying_thrust < 0.0 && engine.direction < 0 && !ship.thrust_failure)
            || (ship.applying_torque > 0.0 && engine.rotate > 0 && !ship.torque_failure)
            || (ship.applying_torque < 0.0 && engine.rotate < 0 && !ship.torque_failure))
    {
        true
    } else {
        false
    }
}

fn emit_particle_for<'a>(
    engine: &'a mut Engine,
    time: &Time,
    lazy_update: &LazyUpdate,
    transform: &Transform,
    mover: &Movable,
    entities: &Read<EntitiesRes>,
    asset_manager: &AssetManager,
) {
    let mut rng = rand::thread_rng();
    engine.last_emit = time.absolute_time_seconds();
    let mut thrust = 10000.0 * time.delta_seconds();
    if engine.direction > 0 {
        thrust *= -1.0;
    }

    let mut thrustvector = Vector3::new(0.0, thrust, 0.0);
    let angle = rng.gen_range(-0.15, 0.15);
    thrustvector =
        UnitQuaternion::from_euler_angles(0.0, 0.0, angle).transform_vector(&thrustvector);
    thrustvector = transform.rotation().transform_vector(&thrustvector);
    let mut pos = transform.clone();
    pos.append_translation(engine.location);
    pos.set_scale(Vector3::new(0.1, 0.1, 1.0));

    emit_particle(
        engine.last_emit,
        rng.gen_range(0.2, 0.3),
        pos,
        mover.velocity + thrustvector,
        Tint(engine.tint.clone()),
        &lazy_update,
        &entities,
        &asset_manager,
    );
}
