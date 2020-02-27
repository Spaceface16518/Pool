use amethyst::ecs::{Component, DenseVecStorage};

pub struct Name(pub String);

impl Component for Name {
    type Storage = DenseVecStorage<Self>;
}
