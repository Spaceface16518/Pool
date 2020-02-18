use std::ops::{Div, Mul};

use amethyst::{core::math::Vector2, prelude::World};

#[derive(Copy, Clone, PartialEq)]
pub struct ArenaBounds {
    width: f32,
    height: f32,
}

#[allow(dead_code)]
impl ArenaBounds {
    pub const fn new_unchecked(width: f32, height: f32) -> Self {
        ArenaBounds { width, height }
    }

    pub fn new(width: f32, height: f32) -> Option<Self> {
        if width > 0.0 && height > 0.0 {
            Some(ArenaBounds::new_unchecked(width, height))
        } else {
            None
        }
    }

    pub fn add_to_world(self, world: &mut World) {
        world.insert(self)
    }

    pub const fn width(self) -> f32 {
        self.width
    }

    pub const fn height(self) -> f32 {
        self.height
    }

    pub fn vector(self) -> Vector2<f32> {
        Vector2::new(self.width, self.height)
    }
}

impl Mul<f32> for ArenaBounds {
    type Output = ArenaBounds;

    fn mul(self, ratio: f32) -> Self::Output {
        ArenaBounds {
            width: self.width * ratio,
            height: self.height * ratio,
        }
    }
}

impl Div<f32> for ArenaBounds {
    type Output = ArenaBounds;

    fn div(self, ratio: f32) -> Self::Output {
        ArenaBounds {
            width: self.width / ratio,
            height: self.height / ratio,
        }
    }
}

impl From<(f32, f32)> for ArenaBounds {
    fn from((width, height): (f32, f32)) -> Self {
        ArenaBounds { width, height }
    }
}

impl Into<(f32, f32)> for ArenaBounds {
    fn into(self) -> (f32, f32) {
        (self.width, self.height)
    }
}
