pub mod arena;
pub mod camera;
pub mod physics;

pub fn negative(f: f32) -> f32 {
    if f.is_sign_negative() {
        f
    } else {
        -f
    }
}

pub fn positive(f: f32) -> f32 {
    if f.is_sign_positive() {
        f
    } else {
        -f
    }
}
