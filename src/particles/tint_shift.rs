use amethyst::{
    core::timing::Time,
    ecs::{Component, Join, Read, RunningTime, System, VecStorage, WriteStorage},
    renderer::{palette::rgb::Rgb, resources::Tint},
};
use rand::Rng;

#[derive(Clone, Eq, PartialEq, Debug)]
pub enum TintShiftDirection {
    Up,
    Down,
}

impl TintShiftDirection {
    pub const TINT_SHIFT: f32 = 0.3;

    pub fn shift(&mut self, rgb: &mut Rgb, fixed_seconds: f32) {
        match self {
            TintShiftDirection::Up => {
                let shifted = rgb.red + TintShiftDirection::TINT_SHIFT * fixed_seconds;
                assign_all_channels(rgb, shifted);
                if shifted >= 1.0 {
                    *self = TintShiftDirection::Down;
                }
            }
            TintShiftDirection::Down => {
                let shifted = rgb.red - TintShiftDirection::TINT_SHIFT * fixed_seconds;
                assign_all_channels(rgb, shifted);
                if shifted <= 0.0 {
                    *self = TintShiftDirection::Up;
                }
            }
        };
    }
}

fn assign_all_channels(rgb: &mut Rgb, val: f32) {
    rgb.red = val;
    rgb.blue = val;
    rgb.green = val;
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
    type SystemData = (
        WriteStorage<'a, Tint>,
        WriteStorage<'a, TintShiftDirection>,
        Read<'a, Time>,
    );

    fn run(&mut self, (mut tints, mut tint_shift_directions, time): Self::SystemData) {
        (&mut tints, &mut tint_shift_directions)
            .join()
            .for_each(|(tint, tint_shift_direction)| {
                tint_shift_direction.shift(&mut tint.0.color, time.delta_seconds())
            })
    }

    fn running_time(&self) -> RunningTime {
        RunningTime::Average
    }
}
