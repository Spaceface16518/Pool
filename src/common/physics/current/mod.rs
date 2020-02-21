use amethyst::{
    core::{timing::Time, Transform},
    ecs::{Join, Read, ReadStorage, System, WriteStorage},
};

use noise::Perlin;

use crate::common::physics::{
    current::{mass::InertialMass, noise_generator::NoiseGenerator},
    velocity::Velocity,
};

pub mod mass;
pub mod noise_generator;

pub struct CurrentSystem;

impl<'s> System<'s> for CurrentSystem {
    #[allow(clippy::type_complexity)]
    type SystemData = (
        ReadStorage<'s, InertialMass>,
        ReadStorage<'s, Transform>,
        WriteStorage<'s, Velocity>,
        Read<'s, NoiseGenerator<Perlin, Perlin>>,
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
                (m.get(), (translation[0], translation[1]), v)
            })
            .map(|(m, (x, y), v)| {
                let delta_v = {
                    // find the force from the noise function
                    let f = noise_generator.magnitude(x, y, time.delta_time());
                    // F = ma ∴ a = F/m
                    // Δv = at = FΔt/m
                    f * time.delta_seconds() / m
                };
                let components = {
                    let theta = noise_generator.direction(x, y, time.delta_time());
                    (theta.cos() * delta_v, theta.sin() * delta_v)
                };
                (components, v)
            })
            .for_each(|((dv_x, dv_y), v)| {
                v.x += dv_x;
                v.y += dv_y;
            });
    }
}
