//! Statistics utilities (stub)
pub fn mean(data: &[f32]) -> f32 {
    let sum: f32 = data.iter().sum();
    sum / (data.len() as f32)
}
