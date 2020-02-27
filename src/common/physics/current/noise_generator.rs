use amethyst::prelude::World;
use noise::{NoiseFn, Perlin, Seedable};

// TODO: caching

#[derive(Default, Clone)]
pub struct NoiseGenerator {
    magnitude: Perlin,
    direction: Perlin,
}

impl NoiseGenerator {
    /// Gets the magnitude at a certain location (`x`, `y`)
    #[inline]
    pub fn direction(&self, x: f32, y: f32) -> f32 {
        self.direction.get([x as f64, y as f64]) as f32
    }
}

impl NoiseGenerator {
    /// Gets the magnitude at a certain location (`x`, `y`)
    #[inline]
    pub fn magnitude(&self, x: f32, y: f32) -> f32 {
        // FIXME: use fastest simplification
        self.magnitude.get([x.round() as f64, y.round() as f64]) as f32
    }
}

impl NoiseGenerator {
    /// Makes a new `NoiseGenerator` from two noise functions.
    #[cold]
    #[allow(dead_code)]
    pub fn new(magnitude: Perlin, direction: Perlin) -> Self {
        NoiseGenerator {
            magnitude,
            direction,
        }
    }
}

impl NoiseGenerator {
    pub fn add_to_world(self, world: &mut World) {
        world.insert(self)
    }
}

impl NoiseGenerator {
    /// Prefer this method if you don't need to customize the generators
    #[cold]
    #[allow(dead_code)]
    pub fn default_seeded(magnitude_seed: u32, direction_seed: u32) -> Self {
        NoiseGenerator::new(
            Perlin::default().set_seed(magnitude_seed),
            Perlin::default().set_seed(direction_seed),
        )
    }
}
