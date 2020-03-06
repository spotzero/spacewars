use amethyst::{
    core::transform::Transform,
    prelude::*,
    renderer::Camera,
};

use crate::{ARENA_HEIGHT, ARENA_WIDTH};

use std::collections::HashMap;

pub struct StatusOfPlayer {
    pub id: u8,
    pub energy: f32,
    pub shields: f32,
    pub hull: f32,
    pub dead: bool,
    pub respawn: f64,
    pub lives: isize,
}

#[derive(Default)]
pub struct StatusOfPlayers {
    pub players: HashMap<u8, StatusOfPlayer>,
}

/// Initialise the camera.
pub fn initialise_camera(world: &mut World) {
    // Setup camera in a way that our screen covers whole arena and (0, 0) is in the bottom left.
    let mut transform = Transform::default();
    transform.set_translation_xyz(ARENA_WIDTH * 0.5, ARENA_HEIGHT * 0.5, 1.0);

    world
        .create_entity()
        .with(Camera::standard_2d(ARENA_WIDTH, ARENA_HEIGHT))
        .with(transform)
        .build();
}