//! Curve utilities (stub)
pub fn bezier(t: f32, p0: f32, p1: f32, p2: f32, p3: f32) -> f32 {
    let u = 1.0 - t;
    u * u * u * p0 + 3.0 * u * u * t * p1 + 3.0 * u * t * t * p2 + t * t * t * p3
}
