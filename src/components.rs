extern crate amethyst;

use amethyst::{
    core::math::Vector3,
    core::transform::Transform,
    ecs::*,
};

#[derive(Component, Debug)]
pub struct Movable {
    pub vel: Transform,
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
    pub applying_thrust: i8,
    pub applying_torque: i8,
}

#[derive(Component, Debug)]
pub struct Lifetime {
    pub start: f32,
    pub end: f32,
}
