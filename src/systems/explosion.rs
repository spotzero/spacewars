use amethyst::{
    core::math::Vector3,
    core::timing::Time,
    core::transform::Transform,
    derive::SystemDesc,
    ecs::prelude::{Join, Read, ReadExpect, ReadStorage, System, SystemData, WriteExpect, WriteStorage},
    ecs::Entities,
};

use crate::{components::*, resources::*};

#[derive(SystemDesc)]
pub struct ExplosionSystem;

impl<'s> System<'s> for ExplosionSystem {
    type SystemData = (
        ReadStorage<'s, Explosion>,
        ReadStorage<'s, Lifetime>,
        WriteStorage<'s, Transform>,
        WriteStorage<'s, Collidable>,
        Read<'s, Time>,
        ReadExpect<'s, Game>,
    );

    fn run(
        &mut self,
        (explosions, lifetimes, mut transforms, mut collidables, time, game): Self::SystemData,
    ) {
        if !game.is_playing() {
            return;
        }

        for (explosion, lifetime, transform, mut collidable) in
            (&explosions, &lifetimes, &mut transforms, &mut collidables).join()
        {
            collidable.radius += explosion.vel * time.delta_seconds();
            let radius =
                (time.absolute_time_seconds() - lifetime.start) * explosion.vel as f64;
            let s = radius / 50.;
            transform.set_scale(Vector3::new(s, s, s));
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
        ReadExpect<'s, Game>,
    );

    fn run(
        &mut self,
        (entities, explosions, mut collision_events, mut damage_events, time, game): Self::SystemData,
    ) {
        if !game.is_playing() {
            return;
        }
        for (entity, explosion) in (&entities, &explosions).join() {
            for i in 0..collision_events.explosion_collisions.len() {
                if entity.id() == collision_events.explosion_collisions[i].explosion {
                    damage_events.events.push(Damage {
                        player: collision_events.explosion_collisions[i].player,
                        damage: explosion.dsp * time.delta_seconds(),
                        kind: damage_types::EXPLOSION,
                    });
                }
            }
        }
        collision_events.explosion_collisions.clear();
    }
}
