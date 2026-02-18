//! OBJ (Wavefront) export for 3D graphs/meshes
use crate::graph3d::{Graph3D, Graph3DNode};
use crate::vec3::Vec3;

pub fn export_obj(graph: &Graph3D) -> String {
    let mut obj = String::new();
    let mut vcount = 1;
    for node in &graph.nodes {
        match node {
            Graph3DNode::Point(p) => {
                obj += &format!("v {} {} {}\n", p.x, p.y, p.z);
                vcount += 1;
            }
            Graph3DNode::Line(a, b) => {
                obj += &format!("v {} {} {}\nv {} {} {}\nl {} {}\n", a.x, a.y, a.z, b.x, b.y, b.z, vcount, vcount + 1);
                vcount += 2;
            }
            Graph3DNode::Polyline(points) | Graph3DNode::Polygon(points) => {
                for p in points {
                    obj += &format!("v {} {} {}\n", p.x, p.y, p.z);
                }
                obj += &format!("l");
                for i in 0..points.len() {
                    obj += &format!(" {}", vcount + i);
                }
                obj += "\n";
                vcount += points.len();
            }
            Graph3DNode::Triangle(a, b, c) => {
                obj += &format!("v {} {} {}\nv {} {} {}\nv {} {} {}\nf {} {} {}\n", a.x, a.y, a.z, b.x, b.y, b.z, c.x, c.y, c.z, vcount, vcount + 1, vcount + 2);
                vcount += 3;
            }
            Graph3DNode::Quad(a, b, c, d) => {
                obj += &format!("v {} {} {}\nv {} {} {}\nv {} {} {}\nv {} {} {}\nf {} {} {} {}\n", a.x, a.y, a.z, b.x, b.y, b.z, c.x, c.y, c.z, d.x, d.y, d.z, vcount, vcount + 1, vcount + 2, vcount + 3);
                vcount += 4;
            }
            Graph3DNode::Mesh { vertices, indices } => {
                for p in vertices {
                    obj += &format!("v {} {} {}\n", p.x, p.y, p.z);
                }
                for tri in indices.chunks(3) {
                    if tri.len() == 3 {
                        obj += &format!("f {} {} {}\n", vcount + tri[0] as usize, vcount + tri[1] as usize, vcount + tri[2] as usize);
                    }
                }
                vcount += vertices.len();
            }
            Graph3DNode::Sphere { center, radius } => {
                obj += &format!("# Sphere at {} {} {} r={}\n", center.x, center.y, center.z, radius);
            }
            Graph3DNode::Box { min, max } => {
                obj += &format!("# Box from {} {} {} to {} {} {}\n", min.x, min.y, min.z, max.x, max.y, max.z);
            }
            Graph3DNode::Image3D { pos, size, .. } => {
                obj += &format!("# 3D image at {} {} {} size {} {} {}\n", pos.x, pos.y, pos.z, size.x, size.y, size.z);
            }
        }
    }
    obj
}
