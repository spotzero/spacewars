use amethyst::{
    core::timing::Time,
    core::transform::Transform,
    core::SystemDesc,
    derive::SystemDesc,
    core::math::Vector3,
    ecs::prelude::{Join, Read, ReadStorage, System, SystemData, World, WriteStorage},
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
        for (ship, transform, movable, energy) in (&mut ships, &transforms, &mut movables, &mut energies).join() {

            if ship.applying_thrust != 0.0 {
                let mut thrust = (ship.thrust * time.delta_seconds()) /  movable.mass;
                if energy.charge > thrust {
                    ship.thrust_failure = false;

                    energy.charge -= thrust * 0.05;

                    if ship.applying_thrust < 0.0 {
                        thrust *= -1.0;
                    }

                    let mut thrustvector = Vector3::new(0.0,thrust,0.0);
                    thrustvector = transform.rotation().transform_vector(&thrustvector);
                    movable.velocity += thrustvector;
                } else {
                    ship.thrust_failure = true;
                }
            }

            if ship.applying_torque != 0.0 {
                let mut torque = (ship.torque * time.delta_seconds()) /  movable.mass;
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
