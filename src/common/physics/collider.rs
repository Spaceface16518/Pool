use amethyst::ecs::{Component, VecStorage};

// TODO: if using distance-squared to compare, store radius-squared
#[derive(Clone, Debug, PartialEq)]
pub struct Collider {
    radius: f32,
}

impl Collider {
    pub const fn radius(&self) -> f32 {
        self.radius
    }

    pub const fn new(radius: f32) -> Self {
        Collider {
            radius
        }
    }
}

impl Component for Collider {
    type Storage = VecStorage<Self>;
}
