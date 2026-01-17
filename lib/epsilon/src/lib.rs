/// Trait for definition of epsilon for value margin
pub trait Epsilon {
    const EPSILON: f32;
}

/// Default precision of Epsilon
pub struct DefaultEpsilon;
impl Epsilon for DefaultEpsilon {
    const EPSILON: f32 = 1e-5;
}

/// High precision of Epsilon
pub struct HighPrecisionEpsitlon;
impl Epsilon for HighPrecisionEpsitlon {
    const EPSILON: f32 = 1e-10;
}

/// Low precision of Epsilon
pub struct LowPrecisionEpsitlon;
impl Epsilon for LowPrecisionEpsitlon {
    const EPSILON: f32 = 1e-3;
}

/// helper with epsilon
pub fn approx_eq<E: Epsilon>(a: f32, b: f32) -> bool {
    (a - b).abs() < E::EPSILON
}

/// helper with epsilon
pub fn approx_zero<E: Epsilon>(v: f32) -> bool {
    v.abs() < E::EPSILON
}
