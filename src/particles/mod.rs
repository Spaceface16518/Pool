use amethyst::core::Transform;
use amethyst::ecs::Component;
use amethyst::ecs::VecStorage;
use amethyst::prelude::{Builder, World, WorldExt};
use amethyst::renderer::palette::Srgba;
use amethyst::renderer::resources::Tint;
use amethyst::renderer::SpriteRender;
use rand::Rng;

use crate::common::arena::ArenaBounds;
use crate::common::physics::velocity::Velocity;
use crate::particles::tint_shift::random_tint_direction;

pub mod tint_shift;

pub struct Particle;

impl Component for Particle {
    type Storage = VecStorage<Self>;
}

pub struct ParticlesConfig<'rng, R> {
    pub rng: &'rng mut R,
    pub bounds: ArenaBounds,
    pub velocity_middle: f32,
    pub velocity_maximum_percent_variation: f32,
    pub sprite_render: SpriteRender,
    pub count: usize,
}

impl<'rng, R: Rng> ParticlesConfig<'rng, R> {
    pub fn add_to_world(self, world: &mut World) {
        (0..self.count).for_each(|_| {
            world
                .create_entity()
                .with(random_transform(self.rng, self.bounds))
                .with(random_initial_tint(self.rng))
                .with(random_tint_direction(self.rng))
                .with(random_particle_velocity(
                    self.rng,
                    self.velocity_middle,
                    self.velocity_maximum_percent_variation,
                ))
                .with(self.sprite_render.clone())
                .build();
        })
    }
}

fn random_transform(rng: &mut (impl Rng + ?Sized), bounds: ArenaBounds) -> Transform {
    let x = rng.gen_range(0.0, bounds.width());
    let y = rng.gen_range(0.0, bounds.height());

    let mut transform = Transform::default();
    transform.set_translation_x(x).set_translation_y(y);
    transform
}

fn random_particle_velocity(
    rng: &mut (impl Rng + ?Sized),
    middle: f32,
    max_percent_variation: f32,
) -> Velocity {
    // yes i know, grow up
    let mut v_gen = || {
        let i = middle + rng.gen_range(-max_percent_variation, max_percent_variation);
        if rng.gen_bool(0.5) {
            i
        } else {
            -i
        }
    };

    Velocity::new(v_gen(), v_gen())
}

fn random_initial_tint(rng: &mut (impl Rng + ?Sized)) -> Tint {
    Tint(Srgba::new(1.0, 1.0, 1.0, rng.gen_range(0f32, 1f32)))
}
