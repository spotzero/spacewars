extern crate amethyst;

use amethyst::{
    core::math::{Vector2, Rotation2},
    ecs::*
};

#[derive(Component, Debug)]
pub struct Movable {
    pub pos: Vector2<f32>, // Position vector.
    pub vel: Vector2<f32>, // Velocity vector.
}

#[derive(Component, Debug)]
pub struct Spinable {
    pub rot: Rotation2<f32>, // Rotation Matrix.
    pub ang: f32, // Angular momentum.
}

#[derive(Component, Debug)]
pub struct Physical {
    pub mass: f32 // Mass.
}

#[derive(Component, Debug)]
pub struct Collidable {
    pub radius: f32, // radius.
}

#[derive(Component, Debug)]
pub struct Flyer {
    pub hull: f32,
    pub shield: f32,
}

#[derive(Component, Debug)]
pub struct Energy {
    pub energy: f32,
    pub recharge_rate: f32,
}

#[derive(Component, Debug)]
pub struct Ship {
    pub controllable: bool
}

#[derive(Component, Debug)]
pub struct Lifetime {
    pub start: f32,
    pub end: f32,
}