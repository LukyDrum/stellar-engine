use crate::math::Vector2;

#[derive(Debug, Clone)]
pub struct Triangle {
    pub points: [Vector2<f32>; 3],
}
