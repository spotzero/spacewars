use amethyst::{
    core::math::Vector3,
    core::timing::Time,
    core::transform::Transform,
    core::SystemDesc,
    derive::SystemDesc,
    ecs::prelude::{Join, Read, ReadStorage, System, SystemData, World, WriteExpect, WriteStorage},
    ecs::{Entities, Entity, LazyUpdate, ReadExpect, world::EntitiesRes},
};

use crate::{
    components::*,
    resources::*,
};

use crate::{ARENA_HEIGHT, ARENA_WIDTH};

#[derive(SystemDesc)]
pub struct PlayerDeathSystem;

impl<'s> System<'s> for PlayerDeathSystem {
    type SystemData = (
        Entities<'s>,
        WriteStorage<'s, Transform>,
        WriteStorage<'s, Movable>,
        WriteStorage<'s, Ship>,
        WriteStorage<'s, Player>,
        ReadExpect<'s, SpriteSheetManager>,
        ReadExpect<'s, LazyUpdate>,
        Read<'s, Time>,
    );

    fn run(&mut self, (
        entities,
        mut transforms,
        mut movables,
        mut ships,
        mut players,
        sprite_sheet_manager,
        lazy_update,
        time,
    ): Self::SystemData) {
        for (transform, movable, ship, player) in (&mut transforms, &mut movables, &mut ships, &mut players).join() {
            if ship.hull <= 0. {
                player.dead = true;
                player.respawn = time.absolute_real_time_seconds() + 3.;
                transform.set_translation_xyz(0., 0., 20.0);
                movable.angular_velocity = 0.;
                movable.velocity = Vector3::new(0.,0.,0.);
                generate_explosion(
                    &transform,
                    &movable,
                    movable.mass,
                    2.,
                    150.,
                    &entities,
                    &sprite_sheet_manager,
                    &lazy_update,
                    &time,
                    Explosion {
                        vel: 150.,
                        dsp: 10.,
                    },
                )
            }
        }
    }
}


#[derive(SystemDesc)]
pub struct PlayerRespawnSystem;

impl<'s> System<'s> for PlayerRespawnSystem {
    type SystemData = (
        WriteStorage<'s, Transform>,
        WriteStorage<'s, Movable>,
        WriteStorage<'s, Energy>,
        WriteStorage<'s, Ship>,
        WriteStorage<'s, Player>,
        Read<'s, Time>,
    );

    fn run(&mut self, (
        mut transforms,
        mut movables,
        mut energies,
        mut ships,
        mut players,
        time,
    ): Self::SystemData) {
        for (transform, movable, energy, ship, player) in (&mut transforms, &mut movables, &mut energies, &mut ships, &mut players).join() {
            if player.dead && player.respawn >= time.absolute_real_time_seconds() {
                player.dead = false;
                ship.hull = 50.;
                ship.shield = 75.;
                energy.charge= 100.;
                movable.angular_velocity = 0.6;
                if player.id == 1 {
                    transform.set_translation_xyz(ARENA_WIDTH/4.0, ARENA_HEIGHT/2.0, 0.0);
                    movable.velocity = Vector3::new(0.,120.,0.);
                } else if player.id == 2 {
                    transform.set_translation_xyz(3.0*(ARENA_WIDTH/4.0), ARENA_HEIGHT/2.0, 0.0);
                    movable.velocity = Vector3::new(0.,-120.,0.);
                }
            }
        }
    }
}