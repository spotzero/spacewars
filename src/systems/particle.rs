use amethyst::{
    core::timing::Time,
    core::transform::Transform,
    core::SystemDesc,
    derive::SystemDesc,
    core::math::Vector3,
    ecs::prelude::{Join, Read, ReadStorage, System, SystemData, World, WriteStorage},
    ecs::{Entities, Entity, LazyUpdate, ReadExpect},
    renderer::{sprite::SpriteSheetHandle, transparent::Transparent, SpriteRender},
};

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
        ReadStorage<'s, Transform>,
        ReadStorage<'s, Lifetime>,
    );

    fn run(&mut self, (transforms, lifetimes): Self::SystemData) {
        // Nothing yet.
    }
}


impl<'s> System<'s> for EngineParticleSystem {
    type SystemData = (
        Entities<'s>,
        ReadStorage<'s, Ship>,
        ReadStorage<'s, Transform>,
        ReadStorage<'s, Movable>,
        ReadStorage<'s, ShipEngines>,
        ReadExpect<'s, SpriteSheetManager>,
        ReadExpect<'s, LazyUpdate>,
    );

    fn run (&mut self, (entities, ships, transforms, movables, engines, sprite_sheet_manager, lazy_update): Self::SystemData) {
        for (ship, transform, mover, engine) in (&ships, &transforms, &movables, &engines).join() {
            if ship.applying_thrust != 0.0 {
                let part: Entity = entities.create();
                lazy_update.insert(part, sprite_sheet_manager.get_render("particles/particle0").unwrap());
                lazy_update.insert(part, transform.clone());
                lazy_update.insert(part, Transparent);
                lazy_update.insert(part, Movable {
                  velocity: mover.velocity.clone(),
                  angular_velocity: 0.0,
                  mass: 0.1,
                });
            }

        }
    }
}
