use amethyst::{
    core::Transform, ecs::Join, ecs::ParJoin, ecs::ReadStorage, ecs::System, ecs::WriteStorage,
};

use crate::common::physics::velocity::Velocity;

pub struct MovementSystem;

impl<'a> System<'a> for MovementSystem {
    type SystemData = (WriteStorage<'a, Transform>, ReadStorage<'a, Velocity>);

    fn run(&mut self, (mut transforms, velocities): Self::SystemData) {
        (&mut transforms, &velocities)
            .join()
            .for_each(|(transform, velocity)| {
                transform.append_translation(velocity.get());
            })
    }
}
