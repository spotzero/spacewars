use amethyst::ecs::prelude::Entity;
use std::collections::HashMap;

pub struct StatusOfPlayer {
    pub energy: f32,
    pub shields: f32,
    pub hull: f32,
}

#[derive(Default)]
pub struct StatusOfPlayers {
    pub players: HashMap<u8, StatusOfPlayer>,
}
