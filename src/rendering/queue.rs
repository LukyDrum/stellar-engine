use crate::rendering::{index::Index, shapes::Shape, vertex::Vertex};

/// To render a shape it must be added to the `RenderQueue`.
/// The shapes added will be drawn to the screen on the next call of `RendererState::render`.
#[derive(Debug, Default)]
pub struct RenderQueue {
    shapes: Vec<Shape>,
}

pub(super) struct RenderBuffers {
    pub(super) vertices: Vec<Vertex>,
    pub(super) indices: Vec<Index>,
}

impl RenderQueue {
    pub fn add(&mut self, shape: Shape) {
        self.shapes.push(shape);
    }

    pub fn clear(&mut self) {
        self.shapes.clear();
    }

    pub(super) fn buffers(&self, screen_width: f32, screen_height: f32) -> RenderBuffers {
        let mut vertices = Vec::new();
        let mut indices = Vec::new();
        let mut next_index = 0;

        for shape in &self.shapes {
            let (local_vertices, local_indices) = match shape {
                Shape::Triangle(triangle) => {
                    let local_vertices = triangle
                        .points
                        .iter()
                        .map(|point| {
                            let position = [
                                (point.x / screen_width) * 2.0 - 1.0,
                                (point.y / screen_height) * 2.0 - 1.0,
                            ];
                            let color = [0.75, 0.75, 0.0]; // TODO: Hardcoded value - change later
                            Vertex { position, color }
                        })
                        .collect::<Vec<_>>();
                    let local_indices = [next_index, next_index + 1, next_index + 2];

                    (local_vertices, local_indices)
                }
            };

            next_index += local_indices.len() as u16;
            vertices.extend_from_slice(&local_vertices);
            indices.extend_from_slice(&local_indices);
        }

        RenderBuffers { vertices, indices }
    }
}
