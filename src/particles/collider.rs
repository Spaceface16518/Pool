use amethyst::{
    core::Transform,
    ecs::{Component, Join, ReadExpect, ReadStorage, System, VecStorage, WriteStorage},
};

use crate::common::{
    arena::ArenaBounds,
    negative,
    physics::{collider::Collider, velocity::Velocity},
    positive,
};

#[derive(Clone, Debug)]
pub struct ParticleCollider(Collider);

impl ParticleCollider {
    pub const fn radius(&self) -> f32 {
        self.0.radius()
    }

    pub fn new(radius: f32) -> Self {
        ParticleCollider(Collider::new(radius))
    }
}

impl Component for ParticleCollider {
    type Storage = VecStorage<Self>;
}

pub struct WrapSystem;

impl<'s> System<'s> for WrapSystem {
    type SystemData = (
        ReadExpect<'s, ArenaBounds>,
        ReadStorage<'s, ParticleCollider>,
        ReadStorage<'s, Transform>,
        WriteStorage<'s, Velocity>,
    );

    fn run(&mut self, (arena_bounds, colliders, transforms, mut velocities): Self::SystemData) {
        (&colliders, &transforms, &mut velocities).join().for_each(
            |(collider, transform, velocity)| {
                let (x, y) = {
                    let translation = transform.translation();
                    (translation.x, translation.y)
                };

                if x + collider.radius() >= arena_bounds.width() {
                    velocity.x = negative(velocity.y);
                } else if x - collider.radius() <= 0.0 {
                    velocity.x = positive(velocity.x);
                }

                if y + collider.radius() >= arena_bounds.height() {
                    velocity.y = negative(velocity.y);
                } else if y - collider.radius() <= 0.0 {
                    velocity.y = positive(velocity.y);
                }
            },
        )
    }
}
