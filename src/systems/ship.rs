use amethyst::{
    core::math::Vector3,
    core::timing::Time,
    core::transform::Transform,
    derive::SystemDesc,
    ecs::prelude::{Join, Read, ReadExpect, ReadStorage, System, SystemData, WriteStorage},
    ecs::Entities,
    renderer::palette::Srgba,
    renderer::resources::Tint,
};

use crate::components::{Energy, Movable, Shield, Ship};
use crate::resources::Game;

#[derive(SystemDesc)]
pub struct ShipSystem;

impl<'s> System<'s> for ShipSystem {
    type SystemData = (
        WriteStorage<'s, Ship>,
        ReadStorage<'s, Transform>,
        WriteStorage<'s, Movable>,
        WriteStorage<'s, Energy>,
        Read<'s, Time>,
        ReadExpect<'s, Game>,
    );

    fn run(
        &mut self,
        (mut ships, transforms, mut movables, mut energies, time, game): Self::SystemData,
    ) {
        if !game.is_playing() {
            return;
        }
        for (ship, transform, movable, energy) in
            (&mut ships, &transforms, &mut movables, &mut energies).join()
        {
            if ship.applying_thrust != 0.0 {
                let thrust = (ship.thrust * ship.applying_thrust * time.delta_seconds()) / movable.mass;
                if energy.charge > thrust {
                    ship.thrust_failure = false;

                    energy.charge -= thrust * 0.05;

                    let mut thrustvector = Vector3::new(0.0, thrust, 0.0);
                    thrustvector = transform.rotation().transform_vector(&thrustvector);
                    movable.velocity += thrustvector;
                } else {
                    ship.thrust_failure = true;
                }
            }

            if ship.applying_torque != 0.0 {
                let torque = (ship.torque * ship.applying_torque * time.delta_seconds()) / movable.mass;
                if energy.charge > torque {
                    energy.charge -= torque * 2.0;
                    ship.thrust_failure = false;
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
        ReadExpect<'s, Game>,
    );

    fn run(
        &mut self,
        (entities, shields, ships, mut transforms, mut tints, game): Self::SystemData,
    ) {
        if !game.is_playing() {
            return;
        }
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
