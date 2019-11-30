use amethyst::ecs::Entity;

pub struct Damage {
    pub target: Entity,
    pub amount: f32,
    pub kind: DamageKind,
}

pub enum DamageKind {
    Explosion,
}

pub struct DamageEvents {
   pub  events: Vec<Damage>,
}
