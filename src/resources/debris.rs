use amethyst::{
    core::math::UnitQuaternion, core::math::Vector3, core::timing::Time,
    core::transform::Transform, ecs::prelude::Read, ecs::world::EntitiesRes, ecs::Entity,
    ecs::LazyUpdate, renderer::debug_drawing::DebugLinesComponent, renderer::palette::Srgba,
    renderer::resources::Tint, renderer::transparent::Transparent,
};
use rand::Rng;

use crate::components::*;
use crate::resources::*;

pub fn generate_debris(
    transform: &Transform,
    mover: &Movable,
    mass: f32,
    max_vel: f32,
    entities: &Read<EntitiesRes>,
    sprite_sheet_manager: &SpriteSheetManager,
    lazy_update: &LazyUpdate,
) {
    let mut rng = rand::thread_rng();
}
