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
    pub kind: CollidableKind,
    pub radius: f32,
}

#[derive(Debug)]
pub enum CollidableKind {
    Debris,
    Explosion,
    GravityWell,
    Ship,
    Torpedo,
}


#[derive(Component, Debug)]
pub struct Energy {
    pub charge: f32,
    pub recharge_rate: f32,
    pub max_charge: f32,
}

#[derive(Component, Debug)]
pub struct Ship {
    pub hull: f32,
    pub shield: f32,
    pub thrust: f32,
    pub torque: f32,
    pub thrust_failure: bool,
    pub torque_failure: bool,
    pub applying_thrust: f32,
    pub applying_torque: f32,
}

#[derive(Component, Debug)]
pub struct Player {
    pub id: u8,
    pub controllable: bool,
    pub last_torpedo: f64,
    pub torpedo_interval: f64,
    pub torpedo_energy: f32,
    pub last_hyperspace: f64,
    pub hyperspace_interval: f64,
}

#[derive(Component, Debug)]
pub struct Lifetime {
    pub start: f64,
    pub life: f64,
}

#[derive(Component, Debug)]
pub struct Explosion {
  pub dsp: f32,
  pub vel: f32,
}

#[derive(Component, Default)]
#[storage(NullStorage)]
pub struct ParticleCom;

#[derive(Component, Default)]
#[storage(NullStorage)]
pub struct DebrisCom;

#[derive(Component, Debug)]
pub struct Torpedo {
    pub damage: f32,
    pub fired: f64,
    pub life: f64,
    pub exploding: bool,
}

#[derive(Clone, Debug)]
pub struct Engine {
    pub location: Vector3<f32>,
    pub direction: i8,
    pub rotate: i8,
    pub tint: Srgba,
    pub last_emit: f64,
    pub emit_rate: f64,
}

#[derive(Component, Debug)]
pub struct ShipEngines {
    pub engines: Vec<Engine>,
}

#[derive(Component, Debug)]
pub struct StatusUi {
    pub data: StatusUiKind,
    pub player: u8,
}

#[derive(Debug)]
pub enum StatusUiKind {
    Energy,
    Shields,
    Hull,
}
