use amethyst::core::math::Vector2;
use amethyst::prelude::World;
use amethyst::prelude::WorldExt;

#[derive(Copy, Clone)]
pub struct ArenaBounds {
    width: f32,
    height: f32,
}

impl ArenaBounds {
    pub fn add_to_world(self, world: &mut World) {
        world.insert(self)
    }

    pub const fn width(&self) -> f32 {
        self.width
    }

    pub const fn height(&self) -> f32 {
        self.height
    }

    pub fn vector(&self) -> Vector2<f32> {
        Vector2::new(self.width, self.height)
    }
}