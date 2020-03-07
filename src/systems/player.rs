use amethyst::{
    core::math::Vector3,
    core::timing::Time,
    core::transform::Transform,
    derive::SystemDesc,
    ecs::prelude::{Join, Read, System, SystemData, WriteExpect, WriteStorage},
    ecs::{world::EntitiesRes, Entities, Entity, LazyUpdate, ReadExpect},
    ui::{Anchor, UiText, UiTransform},
};

use crate::{components::*, resources::*};

#[derive(SystemDesc)]
pub struct PlayerDeathSystem;

impl<'s> System<'s> for PlayerDeathSystem {
    type SystemData = (
        Entities<'s>,
        WriteStorage<'s, Transform>,
        WriteStorage<'s, Movable>,
        WriteStorage<'s, Ship>,
        WriteStorage<'s, Player>,
        ReadExpect<'s, AssetManager>,
        ReadExpect<'s, LazyUpdate>,
        WriteExpect<'s, StatusOfPlayers>,
        WriteExpect<'s, AudioEvents>,
        Read<'s, Time>,
        ReadExpect<'s, Game>,
    );

    fn run(
        &mut self,
        (
            entities,
            mut transforms,
            mut movables,
            mut ships,
            mut players,
            asset_manager,
            lazy_update,
            mut status_of_players,
            mut audio_events,
            time,
            game,
        ): Self::SystemData,
    ) {
        if !game.is_playing() {
            return;
        }
        for (entity, transform, movable, ship, player) in (
            &entities,
            &mut transforms,
            &mut movables,
            &mut ships,
            &mut players,
        )
            .join()
        {
            if ship.hull <= 0. {
                audio_events.events.push(AudioEvent::ExplosionPlayer);
                generate_explosion(
                    &transform,
                    &movable,
                    movable.mass / 2.,
                    1.75,
                    100.,
                    &entities,
                    &asset_manager,
                    &lazy_update,
                    &time,
                    Explosion {
                        vel: 100.,
                        dsp: 10.,
                    },
                    4,
                );
                if ship.shield_entity.is_some() {
                    let _ = entities.delete(ship.shield_entity.unwrap());
                }
                let mut status = status_of_players.players.get_mut(&player.id).unwrap();
                status.dead = true;
                status.respawn = time.absolute_time_seconds() + 3.;
                status.lives -= 1;
                transform.set_translation_xyz(0., 0., 20.0);
                movable.angular_velocity = 0.;
                movable.velocity = Vector3::new(0., 0., 0.);
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
        ReadExpect<'s, AssetManager>,
        ReadExpect<'s, LazyUpdate>,
        WriteExpect<'s, StatusOfPlayers>,
        Read<'s, Time>,
        ReadExpect<'s, Game>,
    );

    fn run(
        &mut self,
        (entities, asset_manager, lazy_update, mut status_of_players, time, game): Self::SystemData,
    ) {
        if !game.is_playing() {
            return;
        }
        for mut status in status_of_players.players.values_mut() {
            if status.dead && status.lives >= 0 && status.respawn <= time.absolute_time_seconds() {
                status.dead = false;
                spawn_player(status.id, &lazy_update, &entities, &asset_manager);
            }
        }
    }
}

#[derive(SystemDesc)]
pub struct PlayerWinnerSystem;

impl<'s> System<'s> for PlayerWinnerSystem {
    type SystemData = (
        Entities<'s>,
        WriteStorage<'s, Player>,
        Read<'s, Time>,
        ReadExpect<'s, AssetManager>,
        ReadExpect<'s, LazyUpdate>,
        WriteExpect<'s, StatusOfPlayers>,
        WriteExpect<'s, Game>,
    );

    fn run(
        &mut self,
        (entities, mut players, time, asset_manager, lazy_update, mut status_of_players, mut game): Self::SystemData,
    ) {
        if !game.is_playing() {
            return;
        }
        if game.game_state == GameState::Tie {
            for player in (&mut players).join() {
                player.controllable = false;
            }
            winner_text(&lazy_update, &entities, 3, &asset_manager);
            return;
        }

        let mut winner = 255;
        let mut loser = 255;
        for status in status_of_players.players.values_mut() {
            if !status.lives >= 0 {
                loser = status.id;
            } else {
                winner = status.id;
            }
        }
        if loser != 255 {
            for player in (&mut players).join() {
                player.controllable = false;
            }
            winner_text(&lazy_update, &entities, winner, &asset_manager);
            game.game_state = GameState::Winner;
            game.end_time = time.absolute_time_seconds();
        }
    }
}

#[derive(SystemDesc)]
pub struct PlayerCollisionResponseSystem;

impl<'s> System<'s> for PlayerCollisionResponseSystem {
    type SystemData = (
        Entities<'s>,
        WriteStorage<'s, Movable>,
        WriteExpect<'s, CollisionEvents>,
        WriteExpect<'s, DamageEvents>,
        ReadExpect<'s, Game>,
    );

    fn run(
        &mut self,
        (entities, mut movables, mut collision_events, mut damage_events, game): Self::SystemData,
    ) {
        if !game.is_playing() {
            return;
        }
        for (entity, movable) in (&entities, &mut movables).join() {
            for i in 0..collision_events.player_collisions.len() {
                if collision_events.player_collisions[i].target == entity {
                    // Adjust the velocity ship.
                    movable.velocity += collision_events.player_collisions[i].force / movable.mass;

                    // Create new damage event.
                    damage_events.events.push(Damage {
                        player: collision_events.player_collisions[i].target.id(),
                        damage: collision_events.player_collisions[i].damage,
                        kind: damage_types::KINETIC,
                    });
                }
            }

            for i in 0..collision_events.gravity_well_collision.len() {
                if collision_events.gravity_well_collision[i].target == entity {
                    damage_events.events.push(Damage {
                        player: collision_events.gravity_well_collision[i].target.id(),
                        damage: 2.,
                        kind: damage_types::GRAVITYWELL,
                    });
                }
            }
        }
        collision_events.player_collisions.clear();
    }
}

fn winner_text(
    lazy_update: &LazyUpdate,
    entities: &Read<EntitiesRes>,
    winner: u8,
    asset_manager: &AssetManager,
) {
    if winner > 3 {
        return;
    }
    let winner = winner - 1;

    let player_name = ["The red ship wins!", "The blue ship wins!", "Tie game!"];
    let player_colour = [[1., 0., 0., 1.], [0., 0., 1., 1.], [1.,1.,1.,1.]];

    let font = asset_manager.font().unwrap();

    let ui: Entity = entities.create();

    lazy_update.insert(
        ui,
        UiTransform::new(
            player_name[winner as usize].to_string(),
            Anchor::Middle,
            Anchor::Middle,
            0.0,
            240.0,
            0.0,
            900.,
            50.,
        ),
    );
    lazy_update.insert(
        ui,
        UiText::new(
            font.clone(),
            player_name[winner as usize].to_string(),
            player_colour[winner as usize],
            50.,
        ),
    );
}
