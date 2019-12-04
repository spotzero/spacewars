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
pub struct DamageSystem;

impl<'s> System<'s> for DamageSystem {
    type SystemData = (
        Entities<'s>,
        WriteStorage<'s, Ship>,
        WriteExpect<'s, DamageEvents>,
    );

    fn run(&mut self, (
        entities,
        mut ships,
        mut damage_events,
    ): Self::SystemData) {
        for (entity, ship) in (&entities, &mut ships).join() {
            for i in 0..damage_events.events.len() {
                if entity.id() == damage_events.events[i].player {
                    ship.hull -= damage_events.events[i].damage;
                }
            }
        }
        damage_events.events.clear();
    }
}