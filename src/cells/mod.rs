#![allow(unused)]

use amethyst::{
    ecs::{Component, Entity, NullStorage},
    prelude::{Builder, World, WorldExt},
};

use crate::{
    cells::{name::Name, score::Score, size::InitialSize},
    common::physics::velocity::Velocity,
};

mod event_handler;
mod name;
mod score;
mod size;

#[derive(Copy, Clone, Debug, Default)]
pub struct Cell;

impl Component for Cell {
    type Storage = NullStorage<Self>;
}

pub struct CellBuilder {
    pub name: Option<String>,
    pub initial_radius: f32,
    pub initial_score: u32,
}

impl Default for CellBuilder {
    fn default() -> Self {
        CellBuilder {
            name: None,
            initial_radius: 25.0,
            initial_score: 1,
        }
    }
}

impl CellBuilder {
    pub fn add_to_world(self, world: &mut World) -> Entity {
        let mut builder = world
            .create_entity()
            .with(Cell)
            .with(InitialSize::new(self.initial_radius))
            .with(Score::new(self.initial_score));
        // TODO: add other non-original Cell components
        // TODO: add sprite stuff to Cell

        if let Some(name) = self.name {
            builder = builder.with(Name::new(name));
        }

        builder.build()
    }
}
