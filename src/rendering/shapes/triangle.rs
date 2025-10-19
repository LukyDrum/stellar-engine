use crate::{
    math::Vector2,
    rendering::{index::Index, shapes::point_to_vertex, vertex::Vertex},
};

#[derive(Debug, Clone)]
pub struct Triangle {
    pub points: [Vector2<f32>; 3],
}

impl Triangle {
    pub(super) fn get_vertices(&self, screen_width: f32, screen_height: f32) -> [Vertex; 3] {
        let color = [1.0, 0.2, 0.0];

        [
            point_to_vertex(self.points[0], screen_width, screen_height, color),
            point_to_vertex(self.points[1], screen_width, screen_height, color),
            point_to_vertex(self.points[2], screen_width, screen_height, color),
        ]
    }

    pub(super) fn get_indices(&self) -> [Index; 3] {
        [0, 1, 2]
    }
}
