pub struct AudioEvents {
    pub events: Vec<AudioEvent>,
}


pub enum AudioEvent {
    Railgun,
    Torpedo,
    DebrisHit,
    ExplosionTorpedo,
    ExplosionPlayer,
    Engine {
        state: bool,
        player: u8,
    },
}

pub struct AudioStatus {
}
