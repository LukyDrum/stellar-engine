use crate::rendering::{index::Index, shapes::Shape, vertex::Vertex};

/// To render a shape it must be added to the `RenderQueue`.
/// The shapes added will be drawn to the screen on the next call of `RendererState::render`.
#[derive(Debug, Default)]
pub struct RenderQueue {
    shapes: Vec<Shape>,
}

/// Returned by `RenderQueue`.
/// Contains vectors of the new vertices and indices.
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
            let local_vertices = shape.get_vertices(screen_width, screen_height);
            let local_indices = shape.get_indices();
            let local_indices = local_indices
                .into_iter()
                .map(move |index| next_index + index);

            next_index += local_vertices.len() as u16;
            vertices.extend(local_vertices);
            indices.extend(local_indices);
        }

        dbg!(&vertices);
        dbg!(&indices);

        RenderBuffers { vertices, indices }
    }
}
