use amethyst::{
    core::timing::Time,
    derive::SystemDesc,
    ecs::prelude::{Join, Read, ReadExpect, System, SystemData, WriteStorage},
};

use crate::components::Energy;
use crate::resources::Game;

#[derive(SystemDesc)]
pub struct RechargeSystem;

impl<'s> System<'s> for RechargeSystem {
    type SystemData = (WriteStorage<'s, Energy>, Read<'s, Time>, ReadExpect<'s, Game>,);

    fn run(&mut self, (mut energies, time, game): Self::SystemData) {
        if !game.is_playing() {
            return;
        }
        for energy in (&mut energies).join() {
            if energy.charge < energy.max_charge {
                energy.charge += energy.recharge_rate * time.delta_seconds();
                if energy.charge > energy.max_charge {
                    energy.charge = energy.max_charge;
                }
            }
        }
    }
}
