use amethyst::ecs::{Component, DenseVecStorage};
use std::ops::{Add, AddAssign, Sub, SubAssign};

#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Debug, Default)]
pub struct Score(pub usize);

impl Component for Score {
    type Storage = DenseVecStorage<Self>;
}

impl Score {
    pub const POINTS_FOR_EATING_PLAYER: usize = 10;
    pub const POINTS_FOR_EATING_FOOD: usize = 5;
}

impl Add<usize> for Score {
    type Output = Self;

    fn add(self, rhs: usize) -> Self::Output {
        Score(self.0.add(rhs))
    }
}

impl AddAssign<usize> for Score {
    fn add_assign(&mut self, rhs: usize) {
        self.0.add_assign(rhs)
    }
}

impl Sub<usize> for Score {
    type Output = Self;

    fn sub(self, rhs: usize) -> Self::Output {
        Score(self.0.sub(rhs))
    }
}

impl SubAssign<usize> for Score {
    fn sub_assign(&mut self, rhs: usize) {
        self.0.sub_assign(rhs)
    }
}
