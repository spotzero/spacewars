use amethyst::{
    core::transform::Transform,
    core::math::Vector3,
    ecs::prelude::Read,
    ecs::{Entity, LazyUpdate, world::EntitiesRes},
    renderer::{transparent::Transparent, resources::Tint},
};

use crate::components::{Lifetime, Movable, ParticleCom};
use crate::resources::{SpriteSheetManager};

pub fn emit_particle(
    emit_time: f64,
    life_time: f64,
    pos: Transform,
    vel: Vector3<f32>,
    colour: Tint,
    lazy_update: &LazyUpdate,
    entities: &Read<EntitiesRes>,
    sprite_sheet_manager: &SpriteSheetManager
)
{
    let part: Entity = entities.create();
    lazy_update.insert(part, sprite_sheet_manager.get_render("particles/particle0").unwrap());
    lazy_update.insert(part, pos);
    lazy_update.insert(part, ParticleCom);
    lazy_update.insert(part, Transparent);
    lazy_update.insert(part, colour);
    lazy_update.insert(part, Lifetime {
        start: emit_time,
        life: life_time,
    });
    lazy_update.insert(part, Movable {
        velocity: vel,
        angular_velocity: 0.0,
        mass: 0.1,
        apply_physics: true,
    });
}

/*
pub fn emit_spark(
    emit_time: f64,
    life_time: f64,
    mut pos: Transform,
    thrust: Vector3<f32>,
    vel: Vector3<f32>,
    colour: Tint,
    lazy_update: &LazyUpdate,
    entities: &Read<EntitiesRes>,
    sprite_sheet_manager: &SpriteSheetManager
) {
    let scale = pos.scale_mut();
    scale[1] = scale[1] * 3.;
    pos.set_rotation_2d(thrust.angle(&Vector3::new(0.,1.,0.)));
    emit_particle(emit_time, life_time, pos, vel + thrust, colour, lazy_update, entities, sprite_sheet_manager);
}
*/
