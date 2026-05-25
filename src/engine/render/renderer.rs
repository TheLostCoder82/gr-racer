use anyhow::Result;
use wgpu::{self, Surface};
use winit::window::Window;
use glam::Mat4;

/// Renderer configuration
pub struct RendererConfig {
    pub width: u32,
    pub height: u32,
    pub vsync: bool,
    pub wireframe: bool,
}

impl Default for RendererConfig {
    fn default() -> Self {
        Self {
            width: 1280,
            height: 720,
            vsync: true,
            wireframe: false,
        }
    }
}

/// Basic vertex for colored geometry
#[repr(C)]
#[derive(Debug, Clone, Copy, bytemuck::Pod, bytemuck::Zeroable)]
pub struct Vertex {
    pub position: [f32; 3],
    pub color: [f32; 3],
}

impl Vertex {
    pub const ATTRIBS: [wgpu::VertexAttribute; 2] = [
        wgpu::VertexAttribute {
            offset: 0,
            shader_location: 0,
            format: wgpu::VertexFormat::Float32x3,
        },
        wgpu::VertexAttribute {
            offset: std::mem::size_of::<[f32; 3]>() as u64,
            shader_location: 1,
            format: wgpu::VertexFormat::Float32x3,
        },
    ];

    pub fn desc() -> wgpu::VertexBufferLayout<'static> {
        wgpu::VertexBufferLayout {
            array_stride: std::mem::size_of::<Vertex>() as wgpu::BufferAddress,
            step_mode: wgpu::VertexStepMode::Vertex,
            attributes: &Self::ATTRIBS,
        }
    }
}

/// Uniform data for MVP matrices
#[repr(C)]
#[derive(Debug, Clone, Copy, bytemuck::Pod, bytemuck::Zeroable)]
pub struct Uniforms {
    pub view_proj: [[f32; 4]; 4],
}

impl Uniforms {
    pub fn new() -> Self {
        Self {
            view_proj: Mat4::IDENTITY.to_cols_array_2d(),
        }
    }

    pub fn update(&mut self, view_proj: Mat4) {
        self.view_proj = view_proj.to_cols_array_2d();
    }
}

/// Main renderer struct with full wgpu pipeline
pub struct Renderer {
    pub instance: wgpu::Instance,
    pub surface: Option<Surface<'static>>,
    pub adapter: Option<wgpu::Adapter>,
    pub device: Option<wgpu::Device>,
    pub queue: Option<wgpu::Queue>,
    pub config: Option<wgpu::SurfaceConfiguration>,
    pub wireframe: bool,
    pub render_pipeline: Option<wgpu::RenderPipeline>,
    pub uniform_buffer: Option<wgpu::Buffer>,
    pub bind_group: Option<wgpu::BindGroup>,
    pub depth_texture: Option<wgpu::TextureView>,
}

impl Renderer {
    pub async fn new(window: &Window) -> Result<Self> {
        let instance = wgpu::Instance::new(wgpu::InstanceDescriptor {
            backends: wgpu::Backends::all(),
            ..Default::default()
        });

        let surface = instance.create_surface(window.clone())?;
        
        let adapter = instance
            .request_adapter(&wgpu::RequestAdapterOptions {
                power_preference: wgpu::PowerPreference::HighPerformance,
                force_fallback_adapter: false,
                compatible_surface: Some(&surface),
            })
            .await
            .or_else(|| {
                instance.request_adapter(&wgpu::RequestAdapterOptions {
                    power_preference: wgpu::PowerPreference::LowPower,
                    force_fallback_adapter: false,
                    compatible_surface: Some(&surface),
                }).blocking()
            })
            .ok_or_else(|| anyhow::anyhow!("Failed to find a suitable GPU adapter"))?;

        let features = wgpu::Features::TEXTURE_COMPRESSION_BC | wgpu::Features::SHADER_F16;
        let limits = wgpu::Limits::default();

        let (device, queue) = adapter
            .request_device(
                &wgpu::DeviceDescriptor {
                    label: Some("Renderer Device"),
                    required_features: features,
                    required_limits: limits,
                    memory_hints: wgpu::MemoryHints::Performance,
                },
                None,
            )
            .await?;

        let surface_caps = surface.get_capabilities(&adapter);
        let surface_format = surface_caps
            .formats
            .iter()
            .copied()
            .find(|f| f.is_srgb())
            .unwrap_or(surface_caps.formats[0]);

        let config = wgpu::SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format: surface_format,
            width: window.inner_size().width.max(1),
            height: window.inner_size().height.max(1),
            present_mode: wgpu::PresentMode::Fifo,
            alpha_mode: surface_caps.alpha_modes[0],
            view_formats: vec![],
            desired_maximum_frame_latency: 2,
        };

        surface.configure(&device, &config);

        // Create shaders
        let shader_module = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("Basic Shader"),
            source: wgpu::ShaderSource::Wgsl(include_str!("shaders/basic.wgsl").into()),
        });

        // Create uniform buffer
        let uniform_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Uniform Buffer"),
            contents: bytemuck::cast_slice(&[Uniforms::new()]),
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
        });

        // Create bind group layout
        let bind_group_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: Some("Bind Group Layout"),
            entries: &[wgpu::BindGroupLayoutEntry {
                binding: 0,
                visibility: wgpu::ShaderStages::VERTEX,
                ty: wgpu::BindingType::Buffer {
                    ty: wgpu::BufferBindingType::Uniform,
                    has_dynamic_offset: false,
                    min_binding_size: None,
                },
                count: None,
            }],
        });

        let bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("Bind Group"),
            layout: &bind_group_layout,
            entries: &[wgpu::BindGroupEntry {
                binding: 0,
                resource: uniform_buffer.as_entire_binding(),
            }],
        });

        // Create render pipeline
        let pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: Some("Pipeline Layout"),
            bind_group_layouts: &[&bind_group_layout],
            push_constant_ranges: &[],
        });

        let render_pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("Render Pipeline"),
            layout: Some(&pipeline_layout),
            vertex: wgpu::VertexState {
                module: &shader_module,
                entry_point: "vs_main",
                buffers: &[Vertex::desc()],
                compilation_options: wgpu::PipelineCompilationOptions::default(),
            },
            fragment: Some(wgpu::FragmentState {
                module: &shader_module,
                entry_point: "fs_main",
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
            depth_stencil: Some(wgpu::DepthStencilState {
                format: wgpu::TextureFormat::Depth32Float,
                depth_write_enabled: true,
                depth_compare: wgpu::CompareFunction::Less,
                stencil: wgpu::StencilState::default(),
                bias: wgpu::DepthBiasState::default(),
            }),
            multisample: wgpu::MultisampleState {
                count: 1,
                mask: !0,
                alpha_to_coverage_enabled: false,
            },
            multiview: None,
            cache: None,
        });

        // Create depth texture
        let depth_texture = Self::create_depth_texture(&device, config.width, config.height);

        Ok(Self {
            instance,
            surface: Some(surface),
            adapter: Some(adapter),
            device: Some(device),
            queue: Some(queue),
            config: Some(config),
            wireframe: false,
            render_pipeline: Some(render_pipeline),
            uniform_buffer: Some(uniform_buffer),
            bind_group: Some(bind_group),
            depth_texture: Some(depth_texture),
        })
    }

    fn create_depth_texture(device: &wgpu::Device, width: u32, height: u32) -> wgpu::TextureView {
        let size = wgpu::Extent3d {
            width,
            height,
            depth_or_array_layers: 1,
        };
        let desc = wgpu::TextureDescriptor {
            label: Some("Depth Texture"),
            size,
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format: wgpu::TextureFormat::Depth32Float,
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            view_formats: &[],
        };
        let texture = device.create_texture(&desc);
        texture.create_view(&wgpu::TextureViewDescriptor::default())
    }

    pub fn resize(&mut self, width: u32, height: u32) {
        if let (Some(ref mut config), Some(ref device), Some(ref surface)) = 
            (&mut self.config, &self.device, &self.surface) 
        {
            config.width = width.max(1);
            config.height = height.max(1);
            surface.configure(device, config);
            
            // Recreate depth texture
            if let Some(ref device) = self.device {
                self.depth_texture = Some(Self::create_depth_texture(device, width.max(1), height.max(1)));
            }
        }
    }

    pub fn begin_render(&mut self) -> Option<wgpu::CommandEncoder> {
        let (device, surface, config) = match (&self.device, &self.surface, &self.config) {
            (Some(d), Some(s), Some(c)) => (d, s, c),
            _ => return None,
        };

        let surface_texture = match surface.get_current_texture() {
            Ok(t) => t,
            Err(_) => return None,
        };

        let view = surface_texture.texture.create_view(&wgpu::TextureViewDescriptor::default());
        
        let mut encoder = device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
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
                            g: 0.1,
                            b: 0.15,
                            a: 1.0,
                        }),
                        store: wgpu::StoreOp::Store,
                    },
                })],
                depth_stencil_attachment: Some(wgpu::RenderPassDepthStencilAttachment {
                    view: self.depth_texture.as_ref()?,
                    depth_ops: Some(wgpu::Operations {
                        load: wgpu::LoadOp::Clear(1.0),
                        store: wgpu::StoreOp::Store,
                    }),
                    stencil_ops: None,
                }),
                timestamp_writes: None,
                occlusion_query_set: None,
            });

            render_pass.set_pipeline(self.render_pipeline.as_ref()?);
            render_pass.set_bind_group(0, self.bind_group.as_ref()?, &[]);
        }

        Some(encoder)
    }

    pub fn draw_mesh(&mut self, encoder: &mut wgpu::CommandEncoder, mesh: &Mesh) {
        let (device, queue) = match (&self.device, &self.queue) {
            (Some(d), Some(q)) => (d, q),
            _ => return,
        };

        // Upload mesh if not already uploaded
        let mut mutable_mesh = mesh.clone();
        if mutable_mesh.vertex_buffer.is_none() {
            mutable_mesh.upload(device, queue);
        }

        // Note: In a real implementation, we'd cache uploaded meshes
        // For now, this is a placeholder for the draw call
        if let (Some(vb), Some(ib)) = (&mutable_mesh.vertex_buffer, &mutable_mesh.index_buffer) {
            // Would set vertex/index buffers and draw here
            let _ = (vb, ib);
        }
    }

    pub fn end_render(&mut self, encoder: wgpu::CommandEncoder) {
        if let Some(surface) = &self.surface {
            let queue = match &self.queue {
                Some(q) => q,
                None => return,
            };
            queue.submit(Some(encoder.finish()));
            surface.get_current_texture().unwrap().present();
        }
    }

    pub fn update_uniforms(&mut self, view_proj: Mat4) {
        if let (Some(ref mut buffer), Some(ref queue)) = (&mut self.uniform_buffer, &self.queue) {
            let mut uniforms = Uniforms::new();
            uniforms.update(view_proj);
            queue.write_buffer(buffer, 0, bytemuck::cast_slice(&[uniforms]));
        }
    }

    pub fn get_device(&self) -> Option<&wgpu::Device> {
        self.device.as_ref()
    }

    pub fn get_queue(&self) -> Option<&wgpu::Queue> {
        self.queue.as_ref()
    }

    pub fn toggle_wireframe(&mut self) {
        self.wireframe = !self.wireframe;
    }
}

/// Simple mesh data
#[derive(Clone)]
pub struct Mesh {
    pub vertices: Vec<Vertex>,
    pub indices: Vec<u32>,
    pub vertex_buffer: Option<wgpu::Buffer>,
    pub index_buffer: Option<wgpu::Buffer>,
    pub index_count: usize,
}

impl Mesh {
    pub fn cube(color: [f32; 3]) -> Self {
        let vertices = vec![
            // Front face
            Vertex { position: [-0.5, -0.5,  0.5], color },
            Vertex { position: [ 0.5, -0.5,  0.5], color },
            Vertex { position: [ 0.5,  0.5,  0.5], color },
            Vertex { position: [-0.5,  0.5,  0.5], color },
            // Back face
            Vertex { position: [-0.5, -0.5, -0.5], color },
            Vertex { position: [-0.5,  0.5, -0.5], color },
            Vertex { position: [ 0.5,  0.5, -0.5], color },
            Vertex { position: [ 0.5, -0.5, -0.5], color },
            // Top face
            Vertex { position: [-0.5,  0.5, -0.5], color },
            Vertex { position: [-0.5,  0.5,  0.5], color },
            Vertex { position: [ 0.5,  0.5,  0.5], color },
            Vertex { position: [ 0.5,  0.5, -0.5], color },
            // Bottom face
            Vertex { position: [-0.5, -0.5, -0.5], color },
            Vertex { position: [ 0.5, -0.5, -0.5], color },
            Vertex { position: [ 0.5, -0.5,  0.5], color },
            Vertex { position: [-0.5, -0.5,  0.5], color },
            // Right face
            Vertex { position: [ 0.5, -0.5, -0.5], color },
            Vertex { position: [ 0.5,  0.5, -0.5], color },
            Vertex { position: [ 0.5,  0.5,  0.5], color },
            Vertex { position: [ 0.5, -0.5,  0.5], color },
            // Left face
            Vertex { position: [-0.5, -0.5, -0.5], color },
            Vertex { position: [-0.5, -0.5,  0.5], color },
            Vertex { position: [-0.5,  0.5,  0.5], color },
            Vertex { position: [-0.5,  0.5, -0.5], color },
        ];

        let indices = vec![
            0,  1,  2,      0,  2,  3,    // front
            4,  5,  6,      4,  6,  7,    // back
            8,  9,  10,     8,  10, 11,   // top
            12, 13, 14,     12, 14, 15,   // bottom
            16, 17, 18,     16,  18, 19,   // right
            20, 21, 22,     20, 22, 23,   // left
        ];

        Self {
            vertices,
            indices,
            vertex_buffer: None,
            index_buffer: None,
            index_count: indices.len(),
        }
    }

    pub fn upload(&mut self, device: &wgpu::Device, queue: &wgpu::Queue) {
        let vertex_data = bytemuck::cast_slice(&self.vertices);
        let index_data = bytemuck::cast_slice(&self.indices);

        self.vertex_buffer = Some(device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Vertex Buffer"),
            contents: vertex_data,
            usage: wgpu::BufferUsages::VERTEX,
        }));

        self.index_buffer = Some(device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Index Buffer"),
            contents: index_data,
            usage: wgpu::BufferUsages::INDEX,
        }));
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_vertex_layout() {
        let desc = Vertex::desc();
        assert_eq!(desc.array_stride, std::mem::size_of::<Vertex>() as wgpu::BufferAddress);
        assert_eq!(desc.attributes.len(), 2);
    }

    #[test]
    fn test_cube_mesh() {
        let mesh = Mesh::cube([1.0, 0.0, 0.0]);
        assert_eq!(mesh.vertices.len(), 24);
        assert_eq!(mesh.indices.len(), 36);
    }

    #[test]
    fn test_uniforms() {
        let mut uniforms = Uniforms::new();
        let view_proj = Mat4::perspective_rh_gl(1.2, 16.0/9.0, 0.1, 100.0);
        uniforms.update(view_proj);
        assert_ne!(uniforms.view_proj, Mat4::IDENTITY.to_cols_array_2d());
    }
}
