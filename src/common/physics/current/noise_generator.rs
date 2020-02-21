use noise::{NoiseFn, Seedable};
use std::time::Duration;

#[derive(Default, Clone)]
pub struct NoiseGenerator<
    MagnitudeGenerator: NoiseFn<[f64; 3]>,
    DirectionGenerator: NoiseFn<[f64; 3]>,
> {
    magnitude: MagnitudeGenerator,
    direction: DirectionGenerator,
}

impl<M: NoiseFn<[f64; 3]>, D: NoiseFn<[f64; 3]>> NoiseGenerator<M, D> {
    /// Gets the magnitude at a certain location (`x`, `y`), and a certain time `t`.
    #[inline]
    pub fn direction(&self, x: f32, y: f32, t: Duration) -> f32 {
        self.direction.get([x as f64, y as f64, t.as_secs_f64()]) as f32
    }
}

impl<M: NoiseFn<[f64; 3]>, D: NoiseFn<[f64; 3]>> NoiseGenerator<M, D> {
    /// Gets the magnitude at a certain location (`x`, `y`), and a certain time `t`.
    #[inline]
    pub fn magnitude(&self, x: f32, y: f32, t: Duration) -> f32 {
        self.magnitude.get([x as f64, y as f64, t.as_secs_f64()]) as f32
    }
}

impl<M: NoiseFn<[f64; 3]>, D: NoiseFn<[f64; 3]>> NoiseGenerator<M, D> {
    /// Makes a new `NoiseGenerator` from two noise functions.
    #[cold]
    #[allow(dead_code)]
    pub fn new(magnitude: M, direction: D) -> Self {
        NoiseGenerator {
            magnitude,
            direction,
        }
    }
}

impl<M: Default + Seedable + NoiseFn<[f64; 3]>, D: Default + Seedable + NoiseFn<[f64; 3]>>
    NoiseGenerator<M, D>
{
    /// Prefer this method if you don't need to customize the generators
    #[cold]
    #[allow(dead_code)]
    pub fn default_seeded(magnitude_seed: u32, direction_seed: u32) -> Self {
        NoiseGenerator::new(
            M::default().set_seed(magnitude_seed),
            D::default().set_seed(direction_seed),
        )
    }
}
