use amethyst::{
    core::math::Vector3,
    core::timing::Time,
    core::transform::Transform,
    core::SystemDesc,
    derive::SystemDesc,
    ecs::prelude::{Join, Read, System, SystemData, World, WriteExpect, WriteStorage},
    ecs::{Entities, LazyUpdate, ReadExpect},
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
        ): Self::SystemData,
    ) {
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
                status.respawn = time.absolute_real_time_seconds() + 3.;
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
    );

    fn run(
        &mut self,
        (
        entities,
        asset_manager,
        lazy_update,
        mut status_of_players,
        time,
    ): Self::SystemData,
    ) {
        for mut status in status_of_players.players.values_mut() {
            if status.dead && status.respawn <= time.absolute_real_time_seconds() {
                status.dead = false;
                spawn_player(status.id, &lazy_update, &entities, &asset_manager);
            }
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
    );

    fn run(
        &mut self,
        (entities, mut movables, mut collision_events, mut damage_events): Self::SystemData,
    ) {
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
                        kind: damage_types::KINETIC,
                    });
                }
            }
        }
        collision_events.player_collisions.clear();
    }
}
