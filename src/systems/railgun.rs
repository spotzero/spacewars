use amethyst::{
    core::math::Vector3,
    core::timing::Time,
    core::transform::Transform,
    derive::SystemDesc,
    ecs::prelude::{Join, Read, ReadStorage, System, SystemData, WriteStorage},
    ecs::{world::EntitiesRes, Entities, Entity, LazyUpdate, ReadExpect, WriteExpect},
    input::{InputHandler, StringBindings},
    renderer::debug_drawing::DebugLinesComponent,
    renderer::transparent::Transparent,
};

use crate::{components::*, resources::*};

#[derive(SystemDesc)]
pub struct FireRailGunSystem;

impl<'s> System<'s> for FireRailGunSystem {
    type SystemData = (
        Entities<'s>,
        ReadStorage<'s, Transform>,
        ReadStorage<'s, Movable>,
        WriteStorage<'s, Player>,
        WriteStorage<'s, Energy>,
        Read<'s, InputHandler<StringBindings>>,
        ReadExpect<'s, AssetManager>,
        ReadExpect<'s, LazyUpdate>,
        Read<'s, Time>,
        WriteExpect<'s, AudioEvents>,
        ReadExpect<'s, Game>,
    );

    fn run(
        &mut self,
        (
            entities,
            transforms,
            movables,
            mut players,
            mut energies,
            input,
            asset_manager,
            lazy_update,
            time,
            mut audio_events,
            game,
        ): Self::SystemData,
    ) {
        if !game.is_playing() {
            return;
        }
        for (transform, movable, player, energy) in
            (&transforms, &movables, &mut players, &mut energies).join()
        {
            let fire_railgun = input
                .action_is_down(&format!("railgun_p{}", player.id))
                .expect("Shoot action exists");
            if fire_railgun
                && player.controllable
                && player.last_railgun + player.railgun_interval < time.absolute_time_seconds()
                && energy.charge > player.railgun_energy
            {
                energy.charge -= player.railgun_energy;
                player.last_railgun = time.absolute_time_seconds();
                audio_events.events.push(AudioEvent::Railgun);
                spawn_railgun(
                    &transform,
                    &movable,
                    &lazy_update,
                    &entities,
                    &asset_manager,
                    &time,
                );
            }
        }
    }
}

fn spawn_railgun(
    transform: &Transform,
    movable: &Movable,
    lazy_update: &LazyUpdate,
    entities: &Read<EntitiesRes>,
    asset_manager: &AssetManager,
    time: &Time,
) {
    let mut thrustvector = Vector3::new(0.0, 600.0, 0.0);
    thrustvector = transform.rotation().transform_vector(&thrustvector);
    let mut pos = transform.clone();
    pos.set_translation(
        pos.translation() + pos.rotation().transform_vector(&Vector3::new(0., 24., 0.)),
    );
    pos.set_scale(Vector3::new(0.06, 0.2, 1.0));
    let part: Entity = entities.create();
    lazy_update.insert(
        part,
        asset_manager.get_render("particles/particle0").unwrap(),
    );
    lazy_update.insert(part, pos);
    lazy_update.insert(part, ParticleCom);
    lazy_update.insert(part, Transparent);
    lazy_update.insert(
        part,
        Collidable {
            kind: collidable_types::DEBRIS,
            radius: 2.0,
            ignore: None,
        },
    );
    lazy_update.insert(part, DebugLinesComponent::with_capacity(16));
    lazy_update.insert(
        part,
        Lifetime {
            start: time.absolute_time_seconds(),
            life: 1.,
        },
    );
    lazy_update.insert(
        part,
        Movable {
            velocity: movable.velocity + thrustvector,
            angular_velocity: 0.,
            mass: 1.0,
            apply_physics: true,
        },
    );
}
