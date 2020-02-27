use std::f32::consts::PI;

use amethyst::{
    core::{timing::Time, Transform},
    ecs::{Join, Read, ReadStorage, RunningTime, System, WriteStorage},
};

use crate::common::physics::{
    current::{mass::InertialMass, noise_generator::NoiseGenerator},
    velocity::Velocity,
};

pub mod drag;
pub mod mass;
pub mod noise_generator;

pub struct CurrentSystem;

impl<'s> System<'s> for CurrentSystem {
    #[allow(clippy::type_complexity)]
    type SystemData = (
        ReadStorage<'s, InertialMass>,
        ReadStorage<'s, Transform>,
        WriteStorage<'s, Velocity>,
        Read<'s, NoiseGenerator>,
        Read<'s, Time>,
    );

    fn run(
        &mut self,
        (inertial_masses, transforms, mut velocities, noise_generator, time): Self::SystemData,
    ) {
        (&inertial_masses, &transforms, &mut velocities)
            .join()
            .map(|(m, pos, v)| {
                let translation = pos.translation();
                (m.0, (translation[0], translation[1]), v)
            })
            .map(|(m, (x, y), v)| {
                let delta_v = {
                    // find the force from the noise function
                    const MAX_FORCE: f32 = 20.0;
                    let f = noise_generator.magnitude(x, y) * MAX_FORCE;
                    // F = ma ∴ a = F/m
                    // Δv = at = FΔt/m
                    f * time.delta_seconds() / m
                };
                let components = {
                    const TAU: f32 = 2.0 * PI;
                    let theta = noise_generator.direction(x, y) * TAU;
                    (delta_v * theta.cos(), delta_v * theta.sin())
                };
                (components, v)
            })
            .for_each(|((delta_v_x, delta_v_y), v)| {
                v.x += delta_v_x;
                v.y += delta_v_y;
            });
    }

    fn running_time(&self) -> RunningTime {
        RunningTime::VeryLong
    }
}
