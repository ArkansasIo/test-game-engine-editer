//! Polygon utilities (stub)
pub fn area(vertices: &[[f32; 2]]) -> f32 {
    let n = vertices.len();
    let mut a = 0.0;
    for i in 0..n {
        let (x0, y0) = (vertices[i][0], vertices[i][1]);
        let (x1, y1) = (vertices[(i + 1) % n][0], vertices[(i + 1) % n][1]);
        a += x0 * y1 - x1 * y0;
    }
    0.5 * a.abs()
}
