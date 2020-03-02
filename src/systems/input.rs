use amethyst::{
    derive::SystemDesc,
    ecs::prelude::{
        Join, Read, ReadExpect, ReadStorage, System, SystemData, WriteExpect, WriteStorage,
    },
    input::{InputHandler, StringBindings},
};

use crate::components::*;
use crate::resources::*;

#[derive(SystemDesc)]
pub struct ShipInputSystem;

impl<'s> System<'s> for ShipInputSystem {
    type SystemData = (
        ReadStorage<'s, Player>,
        WriteStorage<'s, Ship>,
        Read<'s, InputHandler<StringBindings>>,
        WriteExpect<'s, AudioEvents>,
        ReadExpect<'s, Game>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (players, mut ships, input, mut audio_events, game) = data;

        if !game.is_playing() {
            return;
        }

        for (player, ship) in (&players, &mut ships).join() {
            if player.controllable {
                ship.applying_torque = you_mean_one(
                    input
                        .axis_value(&format!("torque_p{}", player.id))
                        .unwrap_or(0.0)
                        + input
                            .axis_value(&format!("pad_torque_p{}", player.id))
                            .unwrap_or(0.0),
                );

                ship.applying_thrust = you_mean_one(
                    input
                        .axis_value(&format!("thrust_p{}", player.id))
                        .unwrap_or(0.0)
                        + input
                            .axis_value(&format!("pad_forward_thrust_p{}", player.id))
                            .unwrap_or(0.0)
                        + (-1.
                            * input
                                .axis_value(&format!("pad_reverse_thrust_p{}", player.id))
                                .unwrap_or(0.0)),
                );

                audio_events.events.push(AudioEvent::Engine {
                    player: player.id,
                    state: ship.applying_thrust != 0. || ship.applying_torque != 0.,
                });
            }
        }
    }
}
