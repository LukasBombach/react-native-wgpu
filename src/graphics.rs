use bytemuck::bytes_of;
use bytemuck::cast_slice;
use bytemuck::Pod;
use bytemuck::Zeroable;
use std::borrow::Cow;
use std::sync::Arc;
use wgpu::util::DeviceExt;
use wgpu::MemoryHints::Performance;
use wgpu::ShaderSource;
use winit::window::Window;

#[repr(C)]
#[derive(Copy, Clone, Debug, Pod, Zeroable)]
pub struct Instance {
    pos: [f32; 2],
    size: [f32; 2],
}

impl Instance {
    pub fn new(x: f32, y: f32, width: f32, height: f32) -> Self {
        Self {
            pos: [x, y],
            size: [width, height],
        }
    }
}

pub struct Gpu<'window> {
    surface: wgpu::Surface<'window>,
    config: wgpu::SurfaceConfiguration,
    device: wgpu::Device,
    queue: wgpu::Queue,
    render_pipeline: wgpu::RenderPipeline,
    vertex_buffer: wgpu::Buffer,
    index_buffer: wgpu::Buffer,
    num_indices: u32,
    instance_buffer: wgpu::Buffer,
    instance_count: u32,
    viewport: [f32; 2],
}

impl<'window> Gpu<'window> {
    pub fn new(window: Arc<Window>, instances: Vec<Instance>) -> Gpu<'window> {
        pollster::block_on(Gpu::new_async(window, instances))
    }

    pub async fn new_async(window: Arc<Window>, instances: Vec<Instance>) -> Gpu<'window> {
        /*
         * window
         */

        let size = window.inner_size();
        let width = size.width.max(1);
        let height = size.height.max(1);
        let viewport = [width as f32, height as f32];

        /*
         * wgpu
         */

        let instance = wgpu::Instance::default();
        let surface = instance.create_surface(Arc::clone(&window)).unwrap();
        let push_const_size = std::mem::size_of::<[f32; 2]>() as u32;

        let adapter = instance
            .request_adapter(&wgpu::RequestAdapterOptions {
                power_preference: wgpu::PowerPreference::default(),
                force_fallback_adapter: false,
                compatible_surface: Some(&surface),
            })
            .await
            .expect("Failed to find an appropriate adapter");

        let (device, queue) = adapter
            .request_device(
                &wgpu::DeviceDescriptor {
                    label: None,
                    required_features: wgpu::Features::PUSH_CONSTANTS,
                    required_limits: wgpu::Limits {
                        max_push_constant_size: push_const_size,
                        ..Default::default()
                    },
                    memory_hints: Performance,
                },
                None,
            )
            .await
            .expect("Failed to create device");

        let config = surface.get_default_config(&adapter, width, height).unwrap();

        surface.configure(&device, &config);

        /*
         * push constants
         */

        let push_constant_range = wgpu::PushConstantRange {
            stages: wgpu::ShaderStages::VERTEX,
            range: 0..push_const_size,
        };

        /*
         * vertices
         */

        let vertices: [[f32; 2]; 4] = [
            [0.0, 1.0], // left top
            [0.0, 0.0], // left bottom
            [1.0, 0.0], // right bottom
            [1.0, 1.0], // right top
        ];
        let indices: [u16; 6] = [0, 1, 2, 0, 2, 3];

        let vertex_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Vertex Buffer"),
            contents: cast_slice(&vertices),
            usage: wgpu::BufferUsages::VERTEX,
        });

        let index_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Index Buffer"),
            contents: cast_slice(&indices),
            usage: wgpu::BufferUsages::INDEX,
        });

        let num_indices = indices.len() as u32;

        /*
         * instances
         */

        let instance_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Instance Buffer"),
            contents: cast_slice(&instances),
            usage: wgpu::BufferUsages::VERTEX,
        });
        let instance_count = instances.len() as u32;

        /*
         * shader
         */

        let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: None,
            source: ShaderSource::Wgsl(Cow::Borrowed(include_str!("shader.wgsl"))),
        });

        /*
         * pipeline
         */

        let render_pipeline_layout =
            device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                label: Some("Render Pipeline Layout"),
                bind_group_layouts: &[],
                push_constant_ranges: &[push_constant_range],
            });

        let render_pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: None,
            layout: Some(&render_pipeline_layout),
            vertex: wgpu::VertexState {
                module: &shader,
                entry_point: Some("vs_main"),
                buffers: &[
                    // Vertex buffer
                    wgpu::VertexBufferLayout {
                        array_stride: std::mem::size_of::<[f32; 2]>() as wgpu::BufferAddress,
                        step_mode: wgpu::VertexStepMode::Vertex,
                        attributes: &[wgpu::VertexAttribute {
                            offset: 0,
                            shader_location: 0,
                            format: wgpu::VertexFormat::Float32x2,
                        }],
                    },
                    // Instance buffer
                    wgpu::VertexBufferLayout {
                        array_stride: std::mem::size_of::<Instance>() as wgpu::BufferAddress,
                        step_mode: wgpu::VertexStepMode::Instance,
                        attributes: &[
                            wgpu::VertexAttribute {
                                offset: 0,
                                shader_location: 1,
                                format: wgpu::VertexFormat::Float32x2,
                            },
                            wgpu::VertexAttribute {
                                offset: std::mem::size_of::<[f32; 2]>() as wgpu::BufferAddress,
                                shader_location: 2,
                                format: wgpu::VertexFormat::Float32x2,
                            },
                        ],
                    },
                ],
                compilation_options: Default::default(),
            },
            fragment: Some(wgpu::FragmentState {
                module: &shader,
                entry_point: Some("fs_main"),
                compilation_options: Default::default(),
                targets: &[Some(config.format.into())],
            }),
            primitive: wgpu::PrimitiveState {
                topology: wgpu::PrimitiveTopology::TriangleList,
                ..Default::default()
            },
            depth_stencil: None,
            multisample: wgpu::MultisampleState::default(),
            multiview: None,
            cache: None,
        });

        Gpu {
            surface,
            config,
            device,
            queue,
            render_pipeline,
            vertex_buffer,
            index_buffer,
            num_indices,
            instance_buffer,
            instance_count,
            viewport,
        }
    }

    pub fn set_size(&mut self, width: u32, height: u32) {
        let width = width.max(1);
        let height = height.max(1);

        self.config.width = width;
        self.config.height = height;
        self.surface.configure(&self.device, &self.config);
        self.viewport = [width as f32, height as f32];
    }

    pub fn draw(&mut self) {
        self.device.poll(wgpu::Maintain::Wait);

        let frame = self
            .surface
            .get_current_texture()
            .expect("Failed to acquire next swap chain texture");

        let view = frame
            .texture
            .create_view(&wgpu::TextureViewDescriptor::default());

        let mut encoder = self
            .device
            .create_command_encoder(&wgpu::CommandEncoderDescriptor {
                label: Some("Draw Encoder"),
            });

        {
            let mut rpass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: None,
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: &view,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(wgpu::Color::BLACK),
                        store: wgpu::StoreOp::Store,
                    },
                })],
                depth_stencil_attachment: None,
                timestamp_writes: None,
                occlusion_query_set: None,
            });

            rpass.set_pipeline(&self.render_pipeline);

            if self.instance_count > 0 {
                rpass.set_push_constants(wgpu::ShaderStages::VERTEX, 0, bytes_of(&self.viewport));
                rpass.set_vertex_buffer(0, self.vertex_buffer.slice(..));
                rpass.set_vertex_buffer(1, self.instance_buffer.slice(..));
                rpass.set_index_buffer(self.index_buffer.slice(..), wgpu::IndexFormat::Uint16);
                rpass.draw_indexed(0..self.num_indices, 0, 0..self.instance_count as _);
            }
        }

        self.queue.submit(Some(encoder.finish()));
        frame.present();
    }
}
