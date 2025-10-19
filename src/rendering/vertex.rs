#[repr(C)]
#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
/// A position on the screen as seen by the GPU.
/// The position has values in range -1.0..1.0.
/// The color is in RGB format with values in range 0.0..1.0.
pub(super) struct Vertex {
    /// Position on the screen with values in the range -1.0..1.0.
    pub(super) position: [f32; 2],
    /// Color in the RGB color format where each value is in range 0.0..1.0.
    pub(super) color: [f32; 3],
}

impl Vertex {
    const ATTRIBS: [wgpu::VertexAttribute; 2] =
        wgpu::vertex_attr_array![0 => Float32x2, 1 => Float32x3];

    pub(super) fn desc() -> wgpu::VertexBufferLayout<'static> {
        wgpu::VertexBufferLayout {
            array_stride: std::mem::size_of::<Vertex>() as wgpu::BufferAddress,
            step_mode: wgpu::VertexStepMode::Vertex,
            attributes: &Self::ATTRIBS,
        }
    }
}
