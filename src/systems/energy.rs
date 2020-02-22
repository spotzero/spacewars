use amethyst::{
    core::timing::Time,
    derive::SystemDesc,
    ecs::prelude::{Join, Read, System, SystemData, WriteStorage},
};

use crate::components::*;

#[derive(SystemDesc)]
pub struct RechargeSystem;

impl<'s> System<'s> for RechargeSystem {
    type SystemData = (WriteStorage<'s, Energy>, Read<'s, Time>);

    fn run(&mut self, (mut energies, time): Self::SystemData) {
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
