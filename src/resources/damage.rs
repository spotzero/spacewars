use amethyst::ecs::Entity;

pub struct Damage {
    pub player: u32,
    pub damage: f32,
    pub kind: u32,
}

pub mod damage_types {
   pub const EXPLOSION: u32 = 1;
}

#[derive(Default)]
pub struct DamageEvents {
   pub  events: Vec<Damage>,
}
