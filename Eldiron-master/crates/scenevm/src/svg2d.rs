//! SVG (Scalable Vector Graphics) export for 2D graphs
use crate::graph2d::{Graph2D, Graph2DNode};

pub fn export_svg(graph: &Graph2D, width: u32, height: u32) -> String {
    let mut svg = format!(
        "<svg xmlns='http://www.w3.org/2000/svg' width='{w}' height='{h}'>\n",
        w = width,
        h = height
    );
    for node in &graph.nodes {
        match node {
            Graph2DNode::Point(p) => {
                svg += &format!("<circle cx='{x}' cy='{y}' r='2' fill='black' />\n", x = p.x, y = p.y);
            }
            Graph2DNode::Line(a, b) => {
                svg += &format!("<line x1='{x1}' y1='{y1}' x2='{x2}' y2='{y2}' stroke='black' />\n", x1 = a.x, y1 = a.y, x2 = b.x, y2 = b.y);
            }
            Graph2DNode::Polyline(points) => {
                let pts: Vec<String> = points.iter().map(|p| format!("{x},{y}", x = p.x, y = p.y)).collect();
                svg += &format!("<polyline points='{}' fill='none' stroke='black' />\n", pts.join(" "));
            }
            Graph2DNode::Polygon(points) => {
                let pts: Vec<String> = points.iter().map(|p| format!("{x},{y}", x = p.x, y = p.y)).collect();
                svg += &format!("<polygon points='{}' fill='gray' stroke='black' />\n", pts.join(" "));
            }
            Graph2DNode::Bezier(p0, p1, p2, p3) => {
                svg += &format!("<path d='M {x0},{y0} C {x1},{y1} {x2},{y2} {x3},{y3}' stroke='black' fill='none' />\n",
                    x0 = p0.x, y0 = p0.y, x1 = p1.x, y1 = p1.y, x2 = p2.x, y2 = p2.y, x3 = p3.x, y3 = p3.y);
            }
            Graph2DNode::Arc(center, radius, start, end) => {
                // Arc as SVG path (approximate)
                let (x, y) = (center.x + radius * start.cos(), center.y + radius * start.sin());
                svg += &format!("<circle cx='{x}' cy='{y}' r='{r}' fill='none' stroke='black' />\n", x = center.x, y = center.y, r = *radius);
            }
            Graph2DNode::Image(pos, size, _data) => {
                svg += &format!("<rect x='{x}' y='{y}' width='{w}' height='{h}' fill='lightgray' />\n", x = pos.x, y = pos.y, w = size.x, h = size.y);
            }
        }
    }
    svg += "</svg>\n";
    svg
}
