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
) {
    println!("Generate debris");
    let mut rng = rand::thread_rng();

    let debris: Entity = entities.create();
    let angle = rng.gen_range(-3.14, 3.14);
    let mut debris_mover = mover.clone();
    debris_mover.velocity += transform.rotation().transform_vector(&UnitQuaternion::from_euler_angles(0.0, 0.0, angle).transform_vector(&Vector3::new( 0.0, rng.gen_range(max_vel / 2., max_vel), 0.0)));
    debris_mover.angular_velocity *= mass / debris_mover.mass;
    debris_mover.mass = mass;
    lazy_update.insert(debris, transform.clone());
    lazy_update.insert(debris, debris_mover);
    lazy_update.insert(
        debris,
        Collidable {
            kind: collidable_types::DEBRIS,
            radius: 0.1,
            ignore: None,
        },
    );
    lazy_update.insert(debris, DebugLinesComponent::with_capacity(16));
    lazy_update.insert(debris, Transparent);
    lazy_update.insert(
        debris,
        sprite_sheet_manager
            .get_render("particles/debris")
            .unwrap(),
    );
}
