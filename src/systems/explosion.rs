use amethyst::{
    core::timing::Time,
    core::SystemDesc,
    derive::SystemDesc,
    ecs::prelude::{Join, Read, ReadStorage, System, SystemData, World, WriteStorage},
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