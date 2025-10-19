use crate::rendering::{index::Index, shapes::triangle::Triangle, vertex::Vertex};

#[derive(Debug)]
pub enum Shape {
    Triangle(Triangle),
}

impl Shape {
    pub(in crate::rendering) fn get_vertices(
        &self,
        screen_width: f32,
        screen_height: f32,
    ) -> Vec<Vertex> {
        match self {
            Shape::Triangle(triangle) => triangle.get_vertices(screen_width, screen_height).into(),
        }
    }

    pub(in crate::rendering) fn get_indices(&self) -> Vec<Index> {
        match self {
            Shape::Triangle(triangle) => triangle.get_indices().into(),
        }
    }
}
