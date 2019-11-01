extern crate amethyst;

use amethyst::{
    core::math::{Rotation2, Vector2},
    ecs::*,
};

#[derive(Component, Debug)]
pub struct Movable {
    pub vel: Vector2<f32>, // Velocity vector.
    pub ang: f32,          // Angular momentum.
    pub mass: f32,         // Mass.
}

#[derive(Component, Debug)]
pub struct Collidable {
    pub radius: f32, // radius.
}

#[derive(Component, Debug)]
pub struct Energy {
    pub energy: f32,
    pub recharge_rate: f32,
}

#[derive(Component, Debug)]
pub struct Ship {
    pub hull: f32,
    pub shield: f32,
    pub thrust: f32,
    pub torque: f32,
    pub controllable: bool,
}

#[derive(Component, Debug)]
pub struct Lifetime {
    pub start: f32,
    pub end: f32,
}
