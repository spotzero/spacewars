use amethyst::{
    core::math::UnitQuaternion, core::math::Vector3, core::timing::Time,
    core::transform::Transform, ecs::prelude::Read, ecs::world::EntitiesRes, ecs::Entity,
    ecs::LazyUpdate, renderer::debug_drawing::DebugLinesComponent, renderer::palette::Srgba,
    renderer::resources::Tint, renderer::transparent::Transparent,
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
    debris: i8,
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
            UnitQuaternion::from_euler_angles(0.0, 0.0, angle).transform_vector(&Vector3::new(
                0.0,
                rng.gen_range(0.0, max_vel),
                0.0,
            ))
        } else {
            flip = true;
            //UnitQuaternion::from_euler_angles(0.0,0.0, angle).transform_vector(&Vector3::new(0.0,max_vel*1.25,0.0))
            UnitQuaternion::from_euler_angles(0.0, 0.0, angle).transform_vector(&Vector3::new(
                0.0,
                rng.gen_range(max_vel / 2., max_vel),
                0.0,
            ))
        };
        thrustvector = transform.rotation().transform_vector(&thrustvector);
        let mut pos = transform.clone();
        pos.append_translation(Vector3::new(0.0, 0.0, 0.2));
        pos.set_scale(Vector3::new(0.1, 0.1, 1.0));

        emit_particle(
            time.absolute_real_time_seconds(),
            rng.gen_range(min_life, max_life),
            pos,
            mover.velocity + thrustvector,
            Tint(Srgba::new(1.0, 0.6, 0.0, 0.5)),
            &lazy_update,
            &entities,
            &sprite_sheet_manager,
        );
    }

    let exploder: Entity = entities.create();
    if debris > 0 {
        let mut debris_count = debris + rng.gen_range(-1, 1);
        while debris_count > 0 {
            debris_count -= 1;
            generate_debris(transform, mover, mass / (2 * debris) as f32, max_vel, entities, sprite_sheet_manager, lazy_update, &exploder);
        }
    }

    lazy_update.insert(exploder, transform.clone());
    lazy_update.insert(exploder, mover.clone());
    lazy_update.insert(
        exploder,
        Collidable {
            kind: collidable_types::EXPLOSION,
            radius: 0.1,
            ignore: None,
        },
    );
    lazy_update.insert(exploder, DebugLinesComponent::with_capacity(16));
    lazy_update.insert(exploder, explosion);
    lazy_update.insert(exploder, Transparent);
    lazy_update.insert(exploder, Tint(Srgba::new(1., 0.6, 0., 0.3)));
    lazy_update.insert(
        exploder,
        sprite_sheet_manager
            .get_render("particles/particle0")
            .unwrap(),
    );
    lazy_update.insert(
        exploder,
        Lifetime {
            start: time.absolute_real_time_seconds(),
            life: max_life,
        },
    );
}
