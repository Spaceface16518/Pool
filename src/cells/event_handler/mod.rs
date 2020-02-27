use amethyst::ecs::{Component, VecStorage};

pub struct EventHandler;

impl Component for EventHandler {
    type Storage = VecStorage<Self>;
}
