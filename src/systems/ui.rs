use amethyst::{
    core::SystemDesc,
    derive::SystemDesc,
    ecs::prelude::{Join, ReadExpect, ReadStorage, System, SystemData, World, WriteStorage, WriteExpect},
    ui::UiText,
};

use crate::components::*;
use crate::resources::*;


#[derive(SystemDesc)]
pub struct StatusUpdateSystem;

impl<'s> System<'s> for StatusUpdateSystem {
    type SystemData = (
        ReadStorage<'s, Player>,
        ReadStorage<'s, Energy>,
        ReadStorage<'s, Ship>,
        WriteExpect<'s, StatusOfPlayers>,
    );

    fn run( &mut self, (players, energies, ships, mut status_of_players): Self::SystemData) {
        for (player, energy, ship) in (&players, &energies, &ships).join() {
            let mut status = status_of_players.players.get_mut(&player.id).unwrap();
            status.energy = energy.charge;
            status.shields = ship.shield;
            status.hull = ship.hull;
        }
    }
}

#[derive(SystemDesc)]
pub struct StatusUiSystem;

impl<'s> System<'s> for StatusUiSystem {
    type SystemData = (
        ReadStorage<'s, StatusUi>,
        ReadExpect<'s, StatusOfPlayers>,
        WriteStorage<'s, UiText>,
    );

    fn run( &mut self, (stat_uis, status_of_players, mut ui_texts): Self::SystemData) {
        for (stat_ui, ui_text) in (&stat_uis, &mut ui_texts).join() {
            let status = status_of_players.players.get(&stat_ui.player).unwrap();

            ui_text.text = match stat_ui.data {
                StatusUiKind::Energy => format!("Energy: {}", status.energy as u8),
                StatusUiKind::Shields => format!("Shields: {}", status.shields as u8),
                StatusUiKind::Hull => format!("Hull: {}", status.hull as u8),
                StatusUiKind::Score => format!("Lives: {}", status.lives as u8),
            };
        }
    }
}
