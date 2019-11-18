use amethyst::{
    core::math::Vector3,
    ecs::Entities,
    ecs::Entity,
    ecs::LazyUpdate,
    world::EntitiesRes,
    renderer::transparent::Transparent,
    renderer::resources::Tint,
    renderer::palette::Srgba,
};

use crate::components::*;

pub fn generate_explosion(
    transform: &Transform,
    mover: &Movable,
    mass: f32,
    entities: &Read<EntitiesRes>,
    sprite_sheet_manager: &SpriteSheetManager
    lazy_update: &LazyUpdate,
    time: &Time,
) {

}