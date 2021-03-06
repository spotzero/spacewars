pub struct Damage {
    pub player: u32,
    pub damage: f32,
    pub kind: u32,
}

pub mod damage_types {
    pub const EXPLOSION: u32 = 1;
    // pub const ENERGY: u32 = 2;
    pub const KINETIC: u32 = 3;
    pub const GRAVITYWELL: u32 = 4;
}

#[derive(Default)]
pub struct DamageEvents {
    pub events: Vec<Damage>,
}

pub fn calculate_damage(
    kind: u32,
    amount: f32,
    mut hull: f32,
    mut shield: f32,
) -> (f32, f32, bool) {
    let mut shielded = true;
    if kind == damage_types::EXPLOSION || kind == damage_types::KINETIC {
        shield -= amount;
        if shield < 0. {
            hull += shield * 2.;
            shielded = false;
        }
    } else {
        //kind == damage_types::ENERGY || kind == damage_types::GRAVITYWELL
        shield -= amount * 2.;
        if shield < 0. {
            hull += shield / 2.;
            shielded = false;
        }
    }

    if shield < 0. {
        shield = 0.;
    }

    (hull, shield, shielded)
}
