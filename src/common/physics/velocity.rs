use std::ops::Mul;

use amethyst::{
    core::{timing::Time, Transform},
    ecs::{Component, Join, Read, ReadStorage, RunningTime, System, VecStorage, WriteStorage},
};

#[derive(Clone, PartialEq, Debug, Default)]
pub struct Velocity {
    pub x: f32,
    pub y: f32,
}

impl Velocity {
    pub fn new(x: f32, y: f32) -> Self {
        Velocity { x, y }
    }
}

impl Component for Velocity {
    type Storage = VecStorage<Self>;
}

impl Mul<f32> for Velocity {
    type Output = Velocity;

    fn mul(self, rhs: f32) -> Self::Output {
        Velocity {
            x: self.x * rhs,
            y: self.y * rhs,
        }
    }
}

pub struct MovementSystem;

impl<'a> System<'a> for MovementSystem {
    type SystemData = (
        WriteStorage<'a, Transform>,
        ReadStorage<'a, Velocity>,
        Read<'a, Time>,
    );

    fn run(&mut self, (mut transforms, velocities, time): Self::SystemData) {
        (&mut transforms, &velocities)
            .join()
            .for_each(|(transform, velocity)| {
                let Velocity { x, y } = velocity.clone() * time.delta_seconds();
                transform.prepend_translation_x(x);
                transform.prepend_translation_y(y);
            })
    }

    fn running_time(&self) -> RunningTime {
        RunningTime::Long
    }
}
