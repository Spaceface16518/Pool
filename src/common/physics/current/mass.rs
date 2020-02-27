use amethyst::ecs::{Component, VecStorage};

#[derive(Clone, Copy, PartialOrd, PartialEq, Debug, Default)]
pub struct InertialMass(pub f32);

impl Component for InertialMass {
    type Storage = VecStorage<Self>;
}
