//! 2D Graph system for procedural and manual shape/image generation
use crate::vec2::Vec2;

#[derive(Debug, Clone)]
pub enum Graph2DNode {
    Point(Vec2),
    Line(Vec2, Vec2),
    Polyline(Vec<Vec2>),
    Polygon(Vec<Vec2>),
    Bezier(Vec2, Vec2, Vec2, Vec2),
    Arc(Vec2, f32, f32, f32), // center, radius, start_angle, end_angle
    Image(Vec2, Vec2, Vec<u8>), // pos, size, raw RGBA
}

#[derive(Debug, Clone)]
pub struct Graph2D {
    pub nodes: Vec<Graph2DNode>,
}

impl Graph2D {
    pub fn new() -> Self {
        Self { nodes: Vec::new() }
    }
    pub fn add(&mut self, node: Graph2DNode) {
        self.nodes.push(node);
    }
    pub fn clear(&mut self) {
        self.nodes.clear();
    }
    // Example: rasterize to image (stub)
    pub fn rasterize(&self, _width: u32, _height: u32) -> Vec<u8> {
        // Implement shape/image rasterization here
        vec![0; (_width * _height * 4) as usize]
    }
}
