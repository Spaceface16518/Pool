use amethyst::ecs::{Component, DenseVecStorage};

use crate::cells::score::Score;

pub struct InitialSize(pub f32);

impl Component for InitialSize {
    type Storage = DenseVecStorage<Self>;
}

impl InitialSize {
    const SCORE_SIZE_SCALE: f32 = 0.1;

    pub fn dynamic_size(&self, score: Score) -> f32 {
        self.0 * score.0 as f32 * Self::SCORE_SIZE_SCALE
    }

    pub const fn new(size: f32) -> Self {
        InitialSize(size)
    }
}
