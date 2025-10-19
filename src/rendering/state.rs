use std::{mem, num::NonZeroU64, sync::Arc};

use wgpu::util::DeviceExt;
use winit::window::Window;

use crate::rendering::{
    index::Index,
    queue::{RenderBuffers, RenderQueue},
    vertex::Vertex,
};

const INITIAL_VERTEX_BUFFER_SIZE: usize = mem::size_of::<Vertex>() * 32;
const INITIAL_INDEX_BUFFER_SIZE: usize = mem::size_of::<Index>() * 32;

pub struct RendererState {
    pub render_queue: RenderQueue,

    surface: wgpu::Surface<'static>,
    device: wgpu::Device,
    queue: wgpu::Queue,
    config: wgpu::SurfaceConfiguration,
    is_surface_configured: bool,
    render_pipeline: wgpu::RenderPipeline,
    vertex_buffer: wgpu::Buffer,
    index_buffer: wgpu::Buffer,
    indices_count: u32,
    window: Arc<Window>,
}

impl RendererState {
    const BUFFER_GROW: usize = 2;

    pub async fn new(window: Arc<Window>) -> anyhow::Result<Self> {
        let size = window.inner_size();

        let instance = wgpu::Instance::new(&wgpu::InstanceDescriptor {
            backends: wgpu::Backends::PRIMARY,
            ..Default::default()
        });

        let surface = instance.create_surface(window.clone())?;

        let adapter = instance
            .request_adapter(&wgpu::RequestAdapterOptions {
                power_preference: wgpu::PowerPreference::default(),
                compatible_surface: Some(&surface),
                force_fallback_adapter: false,
            })
            .await?;

        let (device, queue) = adapter
            .request_device(&wgpu::DeviceDescriptor {
                label: None,
                required_features: wgpu::Features::empty(),
                required_limits: wgpu::Limits::default(),
                memory_hints: Default::default(),
                trace: wgpu::Trace::Off,
            })
            .await?;

        let surface_caps = surface.get_capabilities(&adapter);
        let surface_format = surface_caps
            .formats
            .iter()
            .find(|f| f.is_srgb())
            .copied()
            .unwrap_or(surface_caps.formats[0]);
        let config = wgpu::SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format: surface_format,
            width: size.width,
            height: size.height,
            present_mode: surface_caps.present_modes[0],
            alpha_mode: surface_caps.alpha_modes[0],
            view_formats: vec![],
            desired_maximum_frame_latency: 2,
        };

        let shader = device.create_shader_module(wgpu::include_wgsl!("shaders/shader.wgsl"));
        let render_pipeline_layout =
            device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                label: Some("Render Pipeline Layout"),
                bind_group_layouts: &[],
                push_constant_ranges: &[],
            });
        let render_pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("Render Pipeline"),
            layout: Some(&render_pipeline_layout),
            vertex: wgpu::VertexState {
                module: &shader,
                entry_point: Some("vs_main"),
                buffers: &[Vertex::desc()],
                compilation_options: wgpu::PipelineCompilationOptions::default(),
            },
            fragment: Some(wgpu::FragmentState {
                module: &shader,
                entry_point: Some("fs_main"),
                targets: &[Some(wgpu::ColorTargetState {
                    format: config.format,
                    blend: Some(wgpu::BlendState::REPLACE),
                    write_mask: wgpu::ColorWrites::ALL,
                })],
                compilation_options: wgpu::PipelineCompilationOptions::default(),
            }),
            primitive: wgpu::PrimitiveState {
                topology: wgpu::PrimitiveTopology::TriangleList,
                strip_index_format: None,
                front_face: wgpu::FrontFace::Ccw,
                cull_mode: Some(wgpu::Face::Back),
                polygon_mode: wgpu::PolygonMode::Fill,
                unclipped_depth: false,
                conservative: false,
            },
            depth_stencil: None,
            multisample: wgpu::MultisampleState {
                count: 1,
                mask: !0,
                alpha_to_coverage_enabled: false,
            },
            multiview: None,
            cache: None,
        });

        let vertex_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Main Vertex Buffer"),
            contents: &bytemuck::zeroed_vec(INITIAL_VERTEX_BUFFER_SIZE),
            usage: wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::COPY_DST,
        });

        let index_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Main Index Buffer"),
            contents: &bytemuck::zeroed_vec(INITIAL_INDEX_BUFFER_SIZE),
            usage: wgpu::BufferUsages::INDEX | wgpu::BufferUsages::COPY_DST,
        });

        Ok(Self {
            render_queue: RenderQueue::default(),

            surface,
            device,
            queue,
            config,
            is_surface_configured: false,
            render_pipeline,
            vertex_buffer,
            index_buffer,
            indices_count: 0,
            window,
        })
    }

    pub fn resize(&mut self, width: u32, height: u32) {
        if width > 0 && height > 0 {
            self.config.width = width;
            self.config.height = height;
            self.surface.configure(&self.device, &self.config);
            self.is_surface_configured = true;
        }
    }

    pub fn render(&mut self) -> Result<(), wgpu::SurfaceError> {
        self.window.request_redraw();

        if !self.is_surface_configured {
            return Ok(());
        }

        // Take buffers from `RenderQueue`
        let RenderBuffers { vertices, indices } = self
            .render_queue
            .buffers(self.config.width as f32, self.config.height as f32);
        let vertices_bytes_size = vertices.len() * mem::size_of::<Vertex>();
        let indices_bytes_size = indices.len() * mem::size_of::<Index>();

        // Grow the GPU buffers if needed
        let vertex_buffer_size = self.vertex_buffer.size() as usize;
        if vertices_bytes_size > vertex_buffer_size {
            self.vertex_buffer =
                self.device
                    .create_buffer_init(&wgpu::util::BufferInitDescriptor {
                        label: Some("Main Vertex Buffer"),
                        contents: &bytemuck::zeroed_vec(vertex_buffer_size * Self::BUFFER_GROW),
                        usage: wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::COPY_DST,
                    });
        }

        let index_buffer_size = self.index_buffer.size() as usize;
        if indices_bytes_size > index_buffer_size {
            self.index_buffer = self
                .device
                .create_buffer_init(&wgpu::util::BufferInitDescriptor {
                    label: Some("Main Index Buffer"),
                    contents: &bytemuck::zeroed_vec(index_buffer_size * Self::BUFFER_GROW),
                    usage: wgpu::BufferUsages::INDEX | wgpu::BufferUsages::COPY_DST,
                });
        }

        let nearest_4 = |mut val: usize| {
            while val % 4 != 0 {
                val += 1;
            }
            val
        };

        // Write data into the GPU buffers
        if let Some(size) = NonZeroU64::new(nearest_4(vertices_bytes_size) as u64) {
            if let Some(mut write_view) = self.queue.write_buffer_with(&self.vertex_buffer, 0, size)
            {
                for (buffer_chunk, vertex) in write_view
                    .chunks_mut(mem::size_of::<Vertex>())
                    .zip(vertices.into_iter())
                {
                    buffer_chunk.copy_from_slice(bytemuck::bytes_of(&vertex));
                }
            }
        }
        if let Some(size) = NonZeroU64::new(nearest_4(indices_bytes_size) as u64) {
            self.indices_count = indices.len() as u32;
            if let Some(mut write_view) = self.queue.write_buffer_with(&self.index_buffer, 0, size)
            {
                for (buffer_chunk, index) in write_view
                    .chunks_mut(mem::size_of::<Index>())
                    .zip(indices.into_iter())
                {
                    buffer_chunk.copy_from_slice(bytemuck::bytes_of(&index));
                }
            }
        }
        self.queue.submit([]);

        let output = self.surface.get_current_texture()?;
        let view = output
            .texture
            .create_view(&wgpu::TextureViewDescriptor::default());

        let mut encoder = self
            .device
            .create_command_encoder(&wgpu::CommandEncoderDescriptor {
                label: Some("Render Encoder"),
            });
        {
            let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("Render Pass"),
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: &view,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(wgpu::Color {
                            r: 0.1,
                            g: 0.2,
                            b: 0.3,
                            a: 1.0,
                        }),
                        store: wgpu::StoreOp::Store,
                    },
                    depth_slice: None,
                })],
                depth_stencil_attachment: None,
                occlusion_query_set: None,
                timestamp_writes: None,
            });

            render_pass.set_pipeline(&self.render_pipeline);
            render_pass.set_vertex_buffer(0, self.vertex_buffer.slice(..));
            render_pass.set_index_buffer(self.index_buffer.slice(..), wgpu::IndexFormat::Uint16);
            render_pass.draw_indexed(0..self.indices_count, 0, 0..1);
        }

        self.queue.submit(std::iter::once(encoder.finish()));
        output.present();

        Ok(())
    }

    pub fn window_size(&self) -> (u32, u32) {
        (self.config.width, self.config.height)
    }
}
