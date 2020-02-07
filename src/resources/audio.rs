use std::collections::HashMap;

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
    Engine {
        player: u8,
        state: bool,
    },
}

#[derive(Default)]
pub struct AudioState {
    pub engines: HashMap<u8, bool>,
}
