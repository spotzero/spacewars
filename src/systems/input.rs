use amethyst::{
    core::SystemDesc,
    derive::SystemDesc,
    ecs::prelude::{Join, Read, ReadStorage, System, SystemData, World, WriteStorage},
    input::{InputHandler, StringBindings},
};

use crate::components::*;

#[derive(SystemDesc)]
pub struct ShipInputSystem;

impl<'s> System<'s> for ShipInputSystem {
    type SystemData = (
        ReadStorage<'s, Player>,
        WriteStorage<'s, Ship>,
        Read<'s, InputHandler<StringBindings>>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (players, mut ships, input) = data;

        for (player, ship) in (&players, &mut ships).join() {
            if player.controllable {
                ship.applying_torque = input.axis_value(&format!("torque_p{}", player.id)).unwrap_or(0.0);
                ship.applying_thrust = input.axis_value(&format!("thrust_p{}", player.id)).unwrap_or(0.0);
            }
        }
    }
}