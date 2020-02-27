use amethyst::{
    core::timing::Time,
    ecs::{Component, Join, Read, ReadStorage, RunningTime, System, VecStorage, WriteStorage},
};

use crate::common::physics::{current::mass::InertialMass, velocity::Velocity};

pub struct DragProfile(pub f32);

impl Component for DragProfile {
    type Storage = VecStorage<Self>;
}

pub struct DragSystem;

impl<'s> System<'s> for DragSystem {
    type SystemData = (
        ReadStorage<'s, DragProfile>,
        WriteStorage<'s, Velocity>,
        ReadStorage<'s, InertialMass>,
        Read<'s, Time>,
    );

    fn run(&mut self, (drag_profiles, mut velocities, masses, time): Self::SystemData) {
        (&drag_profiles, &mut velocities, &masses)
            .join()
            .map(|(profile, velocity, mass)| (profile.0, velocity, mass.0))
            .for_each(|(profile, velocity, mass)| {
                let delta_v_dir = |v_dir: f32| {
                    let f = profile * v_dir;
                    (v_dir.abs() - f * time.delta_seconds() / mass).copysign(v_dir)
                };

                velocity.map_each(delta_v_dir);
            })
    }

    fn running_time(&self) -> RunningTime {
        RunningTime::Average
    }
}
