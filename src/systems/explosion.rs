use amethyst::{
    core::timing::Time,
    core::SystemDesc,
    derive::SystemDesc,
    ecs::prelude::{Join, Read, ReadStorage, System, SystemData, World, WriteExpect, WriteStorage},
    ecs::{Entities, Entity, LazyUpdate, ReadExpect, world::EntitiesRes},
};

use crate::{
    components::*,
    resources::*,
};

#[derive(SystemDesc)]
pub struct ExplosionSystem;

impl<'s> System<'s> for ExplosionSystem {
    type SystemData = (
        ReadStorage<'s, Explosion>,
        WriteStorage<'s, Collidable>,
        Read<'s, Time>,
    );

    fn run(&mut self, (
        explosions,
        mut collidables,
        time
    ): Self::SystemData) {
        for (explosion, mut collidable) in (&explosions, &mut collidables).join() {
            collidable.radius += explosion.vel * time.delta_seconds();
        }
    }
}

#[derive(SystemDesc)]
pub struct ExplosionCollisionResponseSystem;

impl<'s> System<'s> for ExplosionCollisionResponseSystem {
    type SystemData = (
        Entities<'s>,
        ReadStorage<'s, Explosion>,
        WriteExpect<'s, CollisionEvents>,
        WriteExpect<'s, DamageEvents>,
        Read<'s, Time>,
    );

    fn run(&mut self, (
        entities,
        explosions,
        mut collision_events,
        mut damage_events,
        time
    ): Self::SystemData) {
        for (entity, explosion) in (&entities, &explosions).join() {
            for i in 0..collision_events.explosion_collisions.len() {
            }
        }
        collision_events.explosion_collisions.clear();
    }
}
