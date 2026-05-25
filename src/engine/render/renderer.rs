use anyhow::Result;
use wgpu::{self, Surface};
use winit::window::Window;

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

/// Main renderer struct
pub struct Renderer {
    pub instance: wgpu::Instance,
    pub surface: Option<Surface<'static>>,
    pub adapter: Option<wgpu::Adapter>,
    pub device: Option<wgpu::Device>,
    pub queue: Option<wgpu::Queue>,
    pub config: Option<wgpu::SurfaceConfiguration>,
    pub wireframe: bool,
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

        Ok(Self {
            instance,
            surface: Some(surface),
            adapter: Some(adapter),
            device: Some(device),
            queue: Some(queue),
            config: Some(config),
            wireframe: false,
        })
    }

    pub fn resize(&mut self, width: u32, height: u32) {
        if let (Some(ref mut config), Some(ref device), Some(ref surface)) = 
            (&mut self.config, &self.device, &self.surface) 
        {
            config.width = width.max(1);
            config.height = height.max(1);
            surface.configure(device, config);
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

/// Simple mesh data
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
            16, 17, 18,     16, 18, 19,   // right
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
}
