pub mod arena;
pub mod physics;

// TODO: make the negative and positive methods better

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
