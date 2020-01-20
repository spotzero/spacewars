use amethyst::{
    core::timing::Time,
    core::transform::Transform,
    core::SystemDesc,
    core::math::Vector3,
    derive::SystemDesc,
    renderer::palette::Srgba,
    ecs::prelude::{Join, Read, ReadStorage, System, SystemData, World, WriteExpect, WriteStorage},
    ecs::{Entities, Entity, LazyUpdate, ReadExpect, world::EntitiesRes},
    input::{InputHandler, StringBindings},
    renderer::{transparent::Transparent},
    renderer::debug_drawing::DebugLinesComponent,
};

use crate::{
    components::*,
    resources::*,
};

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
        ReadExpect<'s, SpriteSheetManager>,
        ReadExpect<'s, LazyUpdate>,
        Read<'s, Time>,
    );

    fn run(&mut self, (
        entities,
        transforms,
        movables,
        mut players,
        mut energies,
        input,
        sprite_sheet_manager,
        lazy_update,
        time
    ): Self::SystemData) {
        for (entity, transform, movable, player, energy) in (&entities, &transforms, &movables, &mut players, &mut energies).join() {
            let fire_railgun = input.action_is_down(&format!("railgun_p{}", player.id)).expect("Shoot action exists");
            if
                fire_railgun
                && player.last_railgun + player.railgun_interval < time.absolute_real_time_seconds()
                && energy.charge > player.railgun_energy
            {
                energy.charge -= player.railgun_energy;
                player.last_railgun = time.absolute_real_time_seconds();
                spawn_railgun(&entity, &transform, &movable, &lazy_update, &entities, &sprite_sheet_manager, &time);
            }

        }
    }
}

fn spawn_railgun(
    entity: &Entity,
    transform: &Transform,
    movable: &Movable,
    lazy_update: &LazyUpdate,
    entities: &Read<EntitiesRes>,
    sprite_sheet_manager: &SpriteSheetManager,
    time: &Time
) {
    let mut thrustvector = Vector3::new(0.0, 600.0,0.0);
    thrustvector = transform.rotation().transform_vector(&thrustvector);
    let mut pos = transform.clone();
    pos.set_translation(pos.translation() + pos.rotation().transform_vector(&Vector3::new(0.,24.,0.)));
    pos.set_scale(Vector3::new(0.06,0.2,1.0));
    let part: Entity = entities.create();
    lazy_update.insert(part, sprite_sheet_manager.get_render("particles/particle0").unwrap());
    lazy_update.insert(part, pos);
    lazy_update.insert(part, ParticleCom);
    lazy_update.insert(part, Transparent);
    lazy_update.insert(part, Collidable {
        kind: collidable_types::DEBRIS,
        radius: 2.0,
        ignore: None,
    });
    lazy_update.insert(part, DebugLinesComponent::with_capacity(16));
    lazy_update.insert(part, Lifetime {
        start: time.absolute_real_time_seconds(),
        life: 1.,
    });
    lazy_update.insert(part, Movable {
        velocity: movable.velocity + thrustvector,
        angular_velocity: 0.,
        mass: 0.5,
        apply_physics: true,
    });
}
