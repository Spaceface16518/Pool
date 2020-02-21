use amethyst::ecs::{Component, VecStorage};

pub struct InertialMass(f32);

impl Component for InertialMass {
    type Storage = VecStorage<Self>;
}
