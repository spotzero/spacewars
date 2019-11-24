use amethyst::ecs::prelude::Entity;
use std::collections::HashMap;

#[derive(Default)]
pub struct StatusUi {
    pub status: HashMap<u8, Entity>
}

pub fn ui_stats_message(
    energy: f32,
    shields: f32,
    hull: f32
) -> String {
  format!("Energy: {}\nShields: {}\nHull: {}", energy, shields, hull).to_string()
}