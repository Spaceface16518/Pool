use amethyst::ecs::{Component, DenseVecStorage};
use std::ops::{Add, AddAssign, Sub, SubAssign};

#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Debug, Default)]
pub struct Score(pub u32);

impl Component for Score {
    type Storage = DenseVecStorage<Self>;
}

impl Score {
    pub const POINTS_FOR_EATING_PLAYER: u32 = 10;
    pub const POINTS_FOR_EATING_FOOD: u32 = 5;

    pub const fn new(score: u32) -> Self {
        Score(score)
    }

    pub const fn get(self) -> u32 {
        self.0
    }
}

impl Add<u32> for Score {
    type Output = Self;

    fn add(self, rhs: u32) -> Self::Output {
        Score(self.0.add(rhs))
    }
}

impl AddAssign<u32> for Score {
    fn add_assign(&mut self, rhs: u32) {
        self.0.add_assign(rhs)
    }
}

impl Sub<u32> for Score {
    type Output = Self;

    fn sub(self, rhs: u32) -> Self::Output {
        Score(self.0.sub(rhs))
    }
}

impl SubAssign<u32> for Score {
    fn sub_assign(&mut self, rhs: u32) {
        self.0.sub_assign(rhs)
    }
}
