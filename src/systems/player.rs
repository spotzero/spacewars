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
pub struct PlayerDeathSystem;

impl<'s> System<'s> for PlayerDeathSystem {
    type SystemData = (
        Entities<'s>,
        WriteStorage<'s, Ship>,
        WriteStorage<'s, Player>,
    );

    fn run(&mut self, (
        entities,
        mut ships,
        mut players,
    ): Self::SystemData) {
    }
}


#[derive(SystemDesc)]
pub struct PlayerRespawnSystem;

impl<'s> System<'s> for PlayerRespawnSystem {
    type SystemData = (
        Entities<'s>,
        WriteStorage<'s, Ship>,
        WriteStorage<'s, Player>,
    );

    fn run(&mut self, (
        entities,
        mut ships,
        mut players,
    ): Self::SystemData) {
    }
}