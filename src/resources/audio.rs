#[derive(Default)]
pub struct AudioEvents {
    pub events: Vec<AudioEvent>,
}

pub enum AudioEvent {
    Railgun,
    Torpedo,
    DebrisHit,
    ShieldHit,
    ExplosionTorpedo,
    ExplosionPlayer,
    Engine,
}