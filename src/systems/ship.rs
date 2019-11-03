use amethyst::{
    core::timing::Time,
    core::transform::Transform,
    core::SystemDesc,
    derive::SystemDesc,
    core::math::Vector3,
    ecs::prelude::{Join, Read, ReadStorage, System, SystemData, World, WriteStorage},
};

use crate::components::*;

/// This system is responsible for moving all balls according to their speed
/// and the time passed.
#[derive(SystemDesc)]
pub struct ShipSystem;

impl<'s> System<'s> for ShipSystem {
    type SystemData = (
        ReadStorage<'s, Ship>,
        ReadStorage<'s, Transform>,
        WriteStorage<'s, Movable>,
        Read<'s, Time>,
    );

    fn run(&mut self, (ships, transforms, mut movables, time): Self::SystemData) {

        for (ship, tranform, movable) in (&ships, &transforms, &mut movables).join() {

            if ship.applying_thrust != 0.0 {
                let mut thrust = (ship.thrust * time.delta_seconds()) /  movable.mass;
                if ship.applying_thrust < 0.0 {
                    thrust *= -1.0;
                }
                
                let mut thrustvector = Vector3::new(0.0,thrust,0.0);
                thrustvector = tranform.rotation().transform_vector(&thrustvector);
                movable.velocity += thrustvector;
            }
            
            if ship.applying_torque != 0.0 {
                let mut torque = (ship.torque * time.delta_seconds()) /  movable.mass;
                if ship.applying_torque < 0.0 {
                    torque *= -1.0;
                }
                movable.angular_velocity += torque;
            }
        }
    }
}