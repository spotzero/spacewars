use amethyst::{
    core::timing::Time,
    core::transform::Transform,
    core::SystemDesc,
    derive::SystemDesc,
    ecs::prelude::{Join, Read, ReadStorage, System, SystemData, World, WriteStorage},
};

use crate::components::*;
use crate::ARENA_HEIGHT;
use crate::ARENA_WIDTH;

#[derive(SystemDesc)]
pub struct GravitywellCollisionSystem;

impl<'s> System<'s> for GravitywellCollisionSystem {
    type SystemData = (
        ReadStorage<'s, Movable>,
        WriteStorage<'s, Transform>,
        ReadStorage<'s, Collidable>,
        Read<'s, Time>,
    );

    fn run(&mut self, (movable, mut transforms, colliables, time): Self::SystemData) {
    }
}
