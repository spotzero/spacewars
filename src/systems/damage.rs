use amethyst::{
    core::SystemDesc,
    derive::SystemDesc,
    ecs::prelude::{Join, System, SystemData, World, WriteExpect, WriteStorage},
    ecs::Entities,
};

use crate::{components::*, resources::*};

#[derive(SystemDesc)]
pub struct DamageSystem;

impl<'s> System<'s> for DamageSystem {
    type SystemData = (
        Entities<'s>,
        WriteStorage<'s, Ship>,
        WriteExpect<'s, DamageEvents>,
        WriteExpect<'s, AudioEvents>,
    );

    fn run(&mut self, (entities, mut ships, mut damage_events, mut audio_events): Self::SystemData) {
        for (entity, ship) in (&entities, &mut ships).join() {
            for i in 0..damage_events.events.len() {
                if entity.id() == damage_events.events[i].player {
                    let event = &damage_events.events[i];
                    let (hull, shield, shielded) =
                        calculate_damage(event.kind, event.damage, ship.hull, ship.shield);
                    ship.hull = hull;
                    ship.shield = shield;
                    if shielded && event.kind == damage_types::KINETIC {
                        audio_events.events.push(AudioEvent::ShieldHit);
                    } else {
                        audio_events.events.push(AudioEvent::HullHit);
                    }
                }
            }
        }
        damage_events.events.clear();
    }
}
