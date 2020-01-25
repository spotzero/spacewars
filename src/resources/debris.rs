use amethyst::{
    core::math::UnitQuaternion, core::math::Vector3,
    core::transform::Transform, ecs::prelude::Read, ecs::world::EntitiesRes, ecs::Entity,
    ecs::LazyUpdate, renderer::debug_drawing::DebugLinesComponent,
    renderer::transparent::Transparent,
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
    exploder: &Entity,
) {
    let mut rng = rand::thread_rng();

    let debris: Entity = entities.create();
    let angle = rng.gen_range(-3.14, 3.14);
    let mut debris_mover = mover.clone();
    let mut debris_transform = transform.clone();
    debris_transform.set_scale(Vector3::new(0.25, 0.25, 1.0));

    debris_mover.velocity += transform.rotation().transform_vector(&UnitQuaternion::from_euler_angles(0.0, 0.0, angle).transform_vector(&Vector3::new( 0.0, rng.gen_range(max_vel / 2., max_vel), 0.0)));
    debris_mover.angular_velocity *= 3.;

    debris_mover.mass = mass;
    lazy_update.insert(debris, debris_transform);
    lazy_update.insert(debris, debris_mover);
    lazy_update.insert(
        debris,
        Collidable {
            kind: collidable_types::DEBRIS,
            radius: 8.0,
            ignore: Some(*exploder),
        },
    );
    lazy_update.insert(debris, DebugLinesComponent::with_capacity(16));
    lazy_update.insert(debris, Transparent);
    lazy_update.insert(
        debris,
        sprite_sheet_manager
            .get_render_sprite("particles/debris", rng.gen_range(0, 7))
            .unwrap(),
    );
}
