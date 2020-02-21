use amethyst::ecs::{Component, VecStorage};

#[derive(Clone, Copy, PartialOrd, PartialEq, Debug, Default)]
pub struct InertialMass(f32);

impl InertialMass {
    pub const fn get(self) -> f32 {
        self.0
    }
}

impl Component for InertialMass {
    type Storage = VecStorage<Self>;
}
