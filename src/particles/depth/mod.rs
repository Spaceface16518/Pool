use rand::Rng;

//pub mod depth_system;
//pub mod depth_variation;

pub fn random_depth(rng: &mut (impl Rng + ?Sized)) -> f32 {
    rng.gen()
}