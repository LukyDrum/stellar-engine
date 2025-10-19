mod shape;
mod triangle;

pub use shape::Shape;
pub use triangle::Triangle;

use crate::{math::Vector2, rendering::vertex::Vertex};

pub(super) fn point_to_vertex(
    point: Vector2<f32>,
    screen_width: f32,
    screen_height: f32,
    color: [f32; 3],
) -> Vertex {
    let x = (point.x / screen_width) * 2.0 - 1.0;
    let y = (point.y / screen_height) * 2.0 - 1.0;

    Vertex {
        position: [x, y],
        color,
    }
}
