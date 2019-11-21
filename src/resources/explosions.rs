use amethyst::{
    core::math::Vector3,
    core::math::UnitQuaternion,
    core::transform::Transform,
    core::timing::Time,
    ecs::LazyUpdate,
    ecs::prelude::Read,
    ecs::world::EntitiesRes,
    renderer::resources::Tint,
    renderer::palette::Srgba,
};
use rand::Rng;

use crate::components::*;
use crate::resources::*;

pub fn generate_explosion(
    transform: &Transform,
    mover: &Movable,
    mass: f32,
    entities: &Read<EntitiesRes>,
    sprite_sheet_manager: &SpriteSheetManager,
    lazy_update: &LazyUpdate,
    time: &Time,
) {

  let mut rng = rand::thread_rng();

  let mut count = 0.0;
  while count < mass {
    count += 0.1;
    let angle = rng.gen_range(-3.14, 3.14);
    let mut thrustvector = UnitQuaternion::from_euler_angles(0.0,0.0, angle).transform_vector(&Vector3::new(0.0,rng.gen_range(0.0, 150.0),0.0));
    thrustvector = transform.rotation().transform_vector(&thrustvector);
    let mut pos = transform.clone();
    pos.append_translation(Vector3::new(0.0,0.0,0.2));
    pos.set_scale(Vector3::new(0.1,0.1,1.0));

    emit_particle(
        time.absolute_real_time_seconds(),
        rng.gen_range(0.6, 1.0),
        pos,
        mover.velocity + thrustvector,
        Tint(Srgba::new(1.0, 0.6, 0.0, 0.5)),
        &lazy_update,
        &entities,
        &sprite_sheet_manager
    );
  }
}
