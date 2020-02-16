use amethyst::core::math::Vector3;
use amethyst::ecs::Component;
use amethyst::ecs::VecStorage;

#[derive(Clone, PartialEq, Debug)]
pub struct Velocity(Vector3<f32>);

impl Velocity {
    pub fn new(x: f32, y: f32) -> Self {
        Velocity(Vector3::new(x, y, 0.0))
    }

    pub fn vector(&self) -> &Vector3<f32> {
        &self.0
    }
}

impl Component for Velocity {
    type Storage = VecStorage<Self>;
}
