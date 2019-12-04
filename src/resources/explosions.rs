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
    ecs::Entity,
};
use rand::Rng;

use crate::components::*;
use crate::resources::*;

pub fn generate_explosion(
    transform: &Transform,
    mover: &Movable,
    mass: f32,
    max_life: f64,
    max_vel: f32,
    entities: &Read<EntitiesRes>,
    sprite_sheet_manager: &SpriteSheetManager,
    lazy_update: &LazyUpdate,
    time: &Time,
    explosion: Explosion,
) {

  let mut rng = rand::thread_rng();

  let mut count = 0.0;
  let mut flip = false;

  let min_life = max_life / 2.;

  while count < mass / 2. {
    count += 0.1;
    let angle = rng.gen_range(-3.14, 3.14);
    let mut thrustvector = if flip {
        flip = false;
        UnitQuaternion::from_euler_angles(0.0,0.0, angle).transform_vector(&Vector3::new(0.0,rng.gen_range(0.0, max_vel),0.0))
    } else {
        flip = true;
        UnitQuaternion::from_euler_angles(0.0,0.0, angle).transform_vector(&Vector3::new(0.0,max_vel,0.0))
    };
    thrustvector = transform.rotation().transform_vector(&thrustvector);
    let mut pos = transform.clone();
    pos.append_translation(Vector3::new(0.0,0.0,0.2));
    pos.set_scale(Vector3::new(0.1,0.1,1.0));

    emit_particle(
        time.absolute_real_time_seconds(),
        rng.gen_range(min_life, max_life),
        pos,
        (mover.velocity / 2.) + thrustvector,
        Tint(Srgba::new(1.0, 0.6, 0.0, 0.5)),
        &lazy_update,
        &entities,
        &sprite_sheet_manager
    );
  }
  let exploder: Entity = entities.create();
  lazy_update.insert(exploder, transform.clone());
  lazy_update.insert(exploder, mover.clone());
  lazy_update.insert(exploder, Collidable { kind: CollidableTypes::EXPLOSION, radius: 0.1});
  lazy_update.insert(exploder, explosion);
  lazy_update.insert(exploder, Lifetime {
      start: time.absolute_real_time_seconds(),
      life: max_life,
  });
}
