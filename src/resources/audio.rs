use amethyst::audio::SourceHandle;
use std::collections::HashMap;
use std::{iter::Cycle, vec::IntoIter};

pub struct Music {
    pub music: Cycle<IntoIter<SourceHandle>>,
}

#[derive(Default)]
pub struct AudioEvents {
    pub events: Vec<AudioEvent>,
}

pub enum AudioEvent {
    Railgun,
    Torpedo,
    HullHit,
    ShieldHit,
    ExplosionTorpedo,
    ExplosionPlayer,
    Engine { player: u8, state: bool },
}

#[derive(Default)]
pub struct AudioState {
    pub engines: HashMap<u8, bool>,
}
