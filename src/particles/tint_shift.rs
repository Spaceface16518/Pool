use amethyst::{
    ecs::{Component, Join, System, VecStorage, WriteStorage},
    renderer::resources::Tint,
};
use rand::Rng;

#[derive(Clone, Eq, PartialEq, Debug)]
pub enum TintShiftDirection {
    Up,
    Down,
}

impl TintShiftDirection {
    pub const TINT_SHIFT: f32 = 0.03;

    pub fn toggle(&mut self) {
        *self = match self {
            TintShiftDirection::Up => TintShiftDirection::Down,
            TintShiftDirection::Down => TintShiftDirection::Up,
        }
    }

    pub fn shift_alpha(&mut self, alpha_channel: &mut f32) {
        match self {
            TintShiftDirection::Up => {
                *alpha_channel += TintShiftDirection::TINT_SHIFT;
                if *alpha_channel >= 1.0 {
                    self.toggle()
                }
            }
            TintShiftDirection::Down => {
                *alpha_channel -= TintShiftDirection::TINT_SHIFT;
                if *alpha_channel <= 0.0 {
                    self.toggle()
                }
            }
        };
    }
}

impl Component for TintShiftDirection {
    type Storage = VecStorage<Self>;
}

pub fn random_tint_direction(rng: &mut (impl Rng + ?Sized)) -> TintShiftDirection {
    if rng.gen_bool(0.5) {
        TintShiftDirection::Up
    } else {
        TintShiftDirection::Down
    }
}

pub struct TintShiftSystem;

impl<'a> System<'a> for TintShiftSystem {
    type SystemData = (WriteStorage<'a, Tint>, WriteStorage<'a, TintShiftDirection>);

    fn run(&mut self, (mut tints, mut tint_shift_directions): Self::SystemData) {
        (&mut tints, &mut tint_shift_directions)
            .join()
            .for_each(|(tint, tint_shift_direction)| {
                let alpha_channel = &mut tint.0.alpha;
                tint_shift_direction.shift_alpha(alpha_channel)
            })
    }
}
