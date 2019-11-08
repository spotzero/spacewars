use amethyst::{
    core::math::Vector3,
    ecs::*,
    renderer::palette::Srgba,
};

#[derive(Component, Debug)]
pub struct Movable {
    pub velocity: Vector3<f32>,
    pub angular_velocity: f32,
    pub mass: f32,
}

#[derive(Component, Debug)]
pub struct Collidable {
    pub radius: f32,
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
    pub applying_thrust: f32,
    pub applying_torque: f32,
}

#[derive(Component, Debug)]
pub struct Player {
    pub id: u8,
    pub controllable: bool,
    pub last_torpedo: f32,
    pub last_missle: f32,
    pub last_hyperspace: f32,
}

#[derive(Component, Debug)]
pub struct Lifetime {
    pub start: f64,
    pub life: f64,
}

#[derive(Clone, Debug)]
pub struct Engine {
    pub location: Vector3<f32>,
    pub direction: bool,
    pub tint: Srgba,
    pub last_emit: f64,
    pub emit_rate: f64,
}

#[derive(Component, Debug)]
pub struct ShipEngines {
    pub engines: Vec<Engine>,
}
