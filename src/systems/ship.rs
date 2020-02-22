use amethyst::{
    core::math::Vector3,
    core::timing::Time,
    core::transform::Transform,
    derive::SystemDesc,
    ecs::prelude::{Join, Read, ReadStorage, System, SystemData, WriteStorage},
    ecs::Entities,
    renderer::palette::Srgba,
    renderer::resources::Tint,
};

use crate::components::*;

#[derive(SystemDesc)]
pub struct ShipSystem;

impl<'s> System<'s> for ShipSystem {
    type SystemData = (
        WriteStorage<'s, Ship>,
        ReadStorage<'s, Transform>,
        WriteStorage<'s, Movable>,
        WriteStorage<'s, Energy>,
        Read<'s, Time>,
    );

    fn run(&mut self, (mut ships, transforms, mut movables, mut energies, time): Self::SystemData) {
        for (ship, transform, movable, energy) in
            (&mut ships, &transforms, &mut movables, &mut energies).join()
        {
            if ship.applying_thrust != 0.0 {
                let mut thrust = (ship.thrust * time.delta_seconds()) / movable.mass;
                if energy.charge > thrust {
                    ship.thrust_failure = false;

                    energy.charge -= thrust * 0.05;

                    if ship.applying_thrust < 0.0 {
                        thrust *= -1.0;
                    }

                    let mut thrustvector = Vector3::new(0.0, thrust, 0.0);
                    thrustvector = transform.rotation().transform_vector(&thrustvector);
                    movable.velocity += thrustvector;
                } else {
                    ship.thrust_failure = true;
                }
            }

            if ship.applying_torque != 0.0 {
                let mut torque = (ship.torque * time.delta_seconds()) / movable.mass;
                if energy.charge > torque {
                    energy.charge -= torque * 2.0;
                    ship.thrust_failure = false;

                    if ship.applying_torque < 0.0 {
                        torque *= -1.0;
                    }
                    movable.angular_velocity += torque;
                } else {
                    ship.torque_failure = true;
                }
            }
        }
    }
}

#[derive(SystemDesc)]
pub struct ShieldSystem;

impl<'s> System<'s> for ShieldSystem {
    type SystemData = (
        Entities<'s>,
        ReadStorage<'s, Shield>,
        ReadStorage<'s, Ship>,
        WriteStorage<'s, Transform>,
        WriteStorage<'s, Tint>,
    );

    fn run(&mut self, (entities, shields, ships, mut transforms, mut tints): Self::SystemData) {
        for (entity, shield, tint) in (&entities, &shields, &mut tints).join() {
            let shield_amount = match ships.get(shield.target) {
                Some(s) => s.shield / s.max_shield,
                None => 0.,
            };

            *tint = Tint(Srgba::new(1.0 - shield_amount, 0.0, shield_amount, 1.));

            let ship_transform = transforms.get(shield.target).cloned().unwrap_or_default();
            let shield_transform = transforms.get_mut(entity).unwrap();
            shield_transform.set_translation(ship_transform.translation().clone());

            if shield_amount <= 0. {
                shield_transform.set_translation_z(100.);
            }
        }
    }
}
