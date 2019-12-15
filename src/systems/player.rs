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
        WriteExpect<'s, StatusOfPlayers>,
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
        mut status_of_players,
        time,
    ): Self::SystemData) {
        for (entity, transform, movable, ship, player) in (&entities, &mut transforms, &mut movables, &mut ships, &mut players).join() {
            if ship.hull <= 0. {
                generate_explosion(
                    &transform,
                    &movable,
                    movable.mass/2.,
                    1.75,
                    100.,
                    &entities,
                    &sprite_sheet_manager,
                    &lazy_update,
                    &time,
                    Explosion {
                        vel: 100.,
                        dsp: 0.,
                    },
                );
                if ship.shield_entity.is_some() {
                    let _ = entities.delete(ship.shield_entity.unwrap());
                }
                let mut status = status_of_players.players.get_mut(&player.id).unwrap();
                status.dead = true;
                status.respawn = time.absolute_real_time_seconds() + 3.;
                status.lives -= 1;
                transform.set_translation_xyz(0., 0., 20.0);
                movable.angular_velocity = 0.;
                movable.velocity = Vector3::new(0.,0.,0.);
                let _ = entities.delete(entity);
            }
        }
    }
}


#[derive(SystemDesc)]
pub struct PlayerRespawnSystem;

impl<'s> System<'s> for PlayerRespawnSystem {
    type SystemData = (
        Entities<'s>,
        ReadExpect<'s, SpriteSheetManager>,
        ReadExpect<'s, LazyUpdate>,
        WriteExpect<'s, StatusOfPlayers>,
        Read<'s, Time>,
    );

    fn run(&mut self, (
        entities,
        sprite_sheet_manager,
        lazy_update,
        mut status_of_players,
        time,
    ): Self::SystemData) {
        for mut status in status_of_players.players.values_mut() {
            if status.dead && status.respawn <= time.absolute_real_time_seconds() {
                status.dead = false;
                spawn_player(status.id, &lazy_update, &entities, &sprite_sheet_manager);
            }
        }
    }
}
