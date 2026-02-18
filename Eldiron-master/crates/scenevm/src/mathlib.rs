//! Math library for 2D/3D game engines: geometry, rendering, graphs, and more
// Inspired by Wolfram library and game engine needs

pub mod vec2;
pub mod vec3;
pub mod mat2;
pub mod mat3;
pub mod mat4;
pub mod quat;
pub mod color;
pub mod aabb;
pub mod ray;
pub mod plane;
pub mod rect;
pub mod transform;
pub mod graph;
pub mod interpolation;
pub mod noise;
pub mod easing;
pub mod projection;
pub mod camera;
pub mod collision;
pub mod curve;
pub mod spline;
pub mod polygon;
pub mod mesh;
pub mod statistics;

// Each submodule should provide core types and functions for its domain.
// For example, vec2.rs: Vec2, dot, cross, length, normalize, etc.
// graph.rs: Graph, Node, Edge, pathfinding, etc.
// interpolation.rs: lerp, slerp, bezier, etc.
// noise.rs: perlin, simplex, value noise, etc.
// collision.rs: AABB, OBB, raycast, intersection tests, etc.
// curve.rs: Bezier, Catmull-Rom, B-spline, etc.
// mesh.rs: Mesh, triangulation, subdivision, etc.
// statistics.rs: mean, median, variance, etc.

// This is a skeleton; implement each module as needed for your engine.
