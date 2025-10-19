use crate::{
    math::Vector2,
    rendering::{index::Index, shapes::point_to_vertex, vertex::Vertex},
};

#[derive(Debug, Clone)]
pub struct Quad {
    pub points: [Vector2<f32>; 4],
}

impl Quad {
    /// Creates a square with all sides equal to `side` with center at `center`.
    /// ```
    /// A ---side--- D
    /// |            |
    /// |     +      |
    /// |            |
    /// B ---------- C
    /// ```
    pub fn square(center: Vector2<f32>, side: f32) -> Self {
        let half_side = side * 0.5;

        Quad {
            points: [
                center + Vector2::new(-half_side, half_side),  // A
                center + Vector2::new(-half_side, -half_side), // B
                center + Vector2::new(half_side, -half_side),  // C
                center + Vector2::new(half_side, half_side),   // D
            ],
        }
    }

    pub(super) fn get_vertices(&self, screen_width: f32, screen_height: f32) -> [Vertex; 4] {
        let color = [1.0, 0.2, 0.0];

        [
            point_to_vertex(self.points[0], screen_width, screen_height, color), // A
            point_to_vertex(self.points[1], screen_width, screen_height, color), // B
            point_to_vertex(self.points[2], screen_width, screen_height, color), // C
            point_to_vertex(self.points[3], screen_width, screen_height, color), // D
        ]
    }

    pub(super) fn get_indices(&self) -> [Index; 6] {
        [
            0, 1, 2, // ABC
            0, 2, 3, // ACD
        ]
    }
}
