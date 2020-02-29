use amethyst::ecs::{Component, DenseVecStorage};

#[derive(Clone, Debug, PartialEq)]
pub struct Name(pub String);

impl Name {
    pub const fn new(name: String) -> Self {
        Name(name)
    }

    pub const fn get(&self) -> &String {
        &self.0
    }
}

impl Component for Name {
    type Storage = DenseVecStorage<Self>;
}
