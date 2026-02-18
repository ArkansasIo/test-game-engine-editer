//! 3D Graph system for procedural and manual shape/image/mesh generation
use crate::vec3::Vec3;

#[derive(Debug, Clone)]
pub enum Graph3DNode {
    Point(Vec3),
    Line(Vec3, Vec3),
    Polyline(Vec<Vec3>),
    Polygon(Vec<Vec3>),
    Triangle(Vec3, Vec3, Vec3),
    Quad(Vec3, Vec3, Vec3, Vec3),
    Mesh {
        vertices: Vec<Vec3>,
        indices: Vec<u32>,
    },
    Sphere { center: Vec3, radius: f32 },
    Box { min: Vec3, max: Vec3 },
    Image3D { pos: Vec3, size: Vec3, data: Vec<u8> }, // 3D image/voxel
}

#[derive(Debug, Clone)]
pub struct Graph3D {
    pub nodes: Vec<Graph3DNode>,
}

mod obj3d;

impl Graph3D {
    pub fn new() -> Self {
        Self { nodes: Vec::new() }
    }
    pub fn add(&mut self, node: Graph3DNode) {
        self.nodes.push(node);
    }
    pub fn clear(&mut self) {
        self.nodes.clear();
    }
    // Example: mesh export (stub)
    pub fn to_mesh(&self) -> (Vec<Vec3>, Vec<u32>) {
        // Implement mesh generation from nodes here
        (Vec::new(), Vec::new())
    }

    /// Export this 3D graph as OBJ format string
    pub fn to_obj(&self) -> String {
        obj3d::export_obj(self)
    }
}
