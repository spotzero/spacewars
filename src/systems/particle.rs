use amethyst::{
    core::timing::Time,
    core::transform::Transform,
    core::SystemDesc,
    derive::SystemDesc,
    core::math::Vector3,
    core::math::UnitQuaternion,
    renderer::palette::Srgba,
    ecs::prelude::{Join, Read, ReadStorage, System, SystemData, World, WriteStorage},
    ecs::{Entities, Entity, LazyUpdate, ReadExpect, world::EntitiesRes},
    renderer::{sprite::SpriteSheetHandle, transparent::Transparent, SpriteRender, resources::Tint},
};

use rand::Rng;

use crate::{
    components::*,
    resources::*,
};

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
    );

    fn run(&mut self, (entities, _particles, lifetimes, time): Self::SystemData) {
        for (entity, lifetime) in (&entities, &lifetimes).join() {
            if lifetime.start + lifetime.life < time.absolute_real_time_seconds() {
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
        ReadExpect<'s, SpriteSheetManager>,
        ReadExpect<'s, LazyUpdate>,
        Read<'s, Time>,
    );

    fn run (&mut self, (entities, ships, transforms, movables, mut ship_engines, sprite_sheet_manager, lazy_update, time): Self::SystemData) {
        for (ship, transform, mover, ship_engine) in (&ships, &transforms, &movables, &mut ship_engines).join() {
            if ship.applying_thrust != 0.0 {
                for i in 0..ship_engine.engines.len() {
                    if check_engine(&ship_engine.engines[i], ship.applying_thrust, &time) {
                        emit_particle_for(&mut ship_engine.engines[i], ship, &time, &lazy_update, transform, mover, &entities, &sprite_sheet_manager);
                    }
                }
            }
        }
    }
}

fn check_engine(engine: &Engine, applying_thrust: f32, time: &Time) -> bool {
    if
        engine.last_emit + engine.emit_rate < time.absolute_real_time_seconds()
        && (
            (applying_thrust > 0.0 && engine.direction)
            ||
            (applying_thrust < 0.0 && !engine.direction)
        )
    {
        true
    } else {
        false
    }
}

fn emit_particle_for<'a>(
    engine: &'a mut Engine,
    ship: &Ship,
    time: &Time,
    lazy_update: &LazyUpdate,
    transform: &Transform,
    mover: &Movable,
    entities: &Read<EntitiesRes>,
    sprite_sheet_manager: &SpriteSheetManager
) {
    let mut rng = rand::thread_rng();
    engine.last_emit = time.absolute_real_time_seconds();
    let mut thrust = (ship.thrust * time.delta_seconds()) / 2.0;
    if ship.applying_thrust > 0.0 {
        thrust *= -1.0;
    }

    let mut thrustvector = Vector3::new(0.0,thrust,0.0);
    let angle = rng.gen_range(-0.15, 0.15);
    thrustvector = UnitQuaternion::from_euler_angles(0.0,0.0, angle).transform_vector(&thrustvector);
    thrustvector = transform.rotation().transform_vector(&thrustvector);
    let mut pos = transform.clone();
    pos.append_translation(engine.location);
    //pos.move_backward(-1.0);
    let part: Entity = entities.create();
    lazy_update.insert(part, sprite_sheet_manager.get_render("particles/particle0").unwrap());
    lazy_update.insert(part, pos);
    lazy_update.insert(part, ParticleCom);
    lazy_update.insert(part, Transparent);
    lazy_update.insert(part, Tint(Srgba::new(1.0, 0.1, 0.1, 0.1)));
    lazy_update.insert(part, Lifetime {
        start: engine.last_emit,
        life: rng.gen_range(0.2, 0.3),
    });
    lazy_update.insert(part, Movable {
        velocity: mover.velocity + thrustvector,
        angular_velocity: 0.0,
        mass: 0.1,
    });
}
