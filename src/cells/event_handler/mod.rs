use crate::cells::event_handler::{action::Action, condition::ConditionExpression};
use amethyst::ecs::{Component, VecStorage};

mod action;
mod condition;

pub struct EventHandler;

impl Component for EventHandler {
    type Storage = VecStorage<Self>;
}

#[derive(Debug, Clone)]
pub struct Handler {
    pub condition: ConditionExpression,
    pub actions: Box<[Action]>,
}

#[derive(Clone, Copy, Debug, PartialEq, Hash)]
pub enum Target {
    Family,
    Strangers,
    Nearest,
}

#[derive(Clone, Copy, Debug, PartialEq, Hash)]
pub enum SizeComparator {
    Equal,
    Larger,
    Smaller,
}
