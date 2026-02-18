//! Geometry toolset for Eldiron: Vertex, Linedef, Sector, Rect, Tile Application

use crate::{
    GeoId, Poly2D, Poly3D, Chunk,
};
use uuid::Uuid;
use vek::{Vec2, Vec3};

/// Tool for creating and manipulating vertices
pub struct VertexTool;

impl VertexTool {
    pub fn add_vertex(chunk: &mut Chunk, pos: [f32; 2]) -> GeoId {
        // In Eldiron, vertices are implicit, but you can track them by GeoId::Vertex
        let id = GeoId::Vertex(rand::random());
        // Optionally store vertex positions in a custom map if needed
        id
    }
}

/// Tool for creating and manipulating linedefs (edges between vertices)
pub struct LinedefTool;

impl LinedefTool {
    pub fn add_linedef(chunk: &mut Chunk, v0: [f32; 2], v1: [f32; 2], tile_id: Uuid, width: f32, layer: i32) -> GeoId {
        let id = GeoId::Linedef(rand::random());
        chunk.add_line_strip_2d(id, tile_id, vec![v0, v1], width, layer);
        id
    }
}

/// Tool for creating and manipulating sectors (closed polygons)
pub struct SectorTool;

impl SectorTool {
    pub fn add_sector(chunk: &mut Chunk, vertices: Vec<[f32; 2]>, tile_id: Uuid, layer: i32) -> GeoId {
        let id = GeoId::Sector(rand::random());
        // Triangulate polygon (simple fan for convex)
        let mut indices = Vec::new();
        for i in 1..vertices.len() - 1 {
            indices.push((0, i, i + 1));
        }
        let uvs = vertices.iter().map(|_| [0.0, 0.0]).collect();
        let poly = Poly2D {
            id,
            tile_id,
            vertices,
            uvs,
            indices,
            transform: vek::Mat3::identity(),
            layer,
            visible: true,
        };
        chunk.add(poly);
        id
    }
}

/// Tool for creating a rectangular sector and applying a tile
pub struct RectTool;

impl RectTool {
    pub fn add_rect(chunk: &mut Chunk, center: [f32; 2], size: f32, tile_id: Uuid, layer: i32) -> GeoId {
        let id = GeoId::Sector(rand::random());
        chunk.add_square_2d(id, tile_id, center, size, layer, true);
        id
    }
}

/// Tool for applying a tile to a sector
pub struct TileTool;

impl TileTool {
    pub fn apply_tile(chunk: &mut Chunk, sector_id: GeoId, tile_id: Uuid) {
        if let Some(poly) = chunk.polys_map.get_mut(&sector_id) {
            poly.tile_id = tile_id;
        }
    }
}
