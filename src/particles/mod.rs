use amethyst::ecs::Component;
use amethyst::ecs::VecStorage;
use amethyst::prelude::World;
use rand::Rng;

pub struct Particle;

impl Component for Particle {
    type Storage = VecStorage<Self>;
}

pub fn generate_particles(rng: &mut (impl Rng + ?Sized)) {
    unimplemented!()
}
