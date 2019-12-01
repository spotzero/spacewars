use amethyst::ecs::Entity;

pub struct Damage {
    pub target: u32,
    pub amount: f32,
    pub kind: u32,
}

pub mod DamageTypes {
   pub const EXPLOSION: u32 = 1;
}

pub struct DamageEvents {
   pub  events: Vec<Damage>,
}
