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
#[derive(Debug, Copy, Clone, Pod, Zeroable)]
struct Uniforms {
    screen_size: [f32; 2],
}

#[repr(C)]
#[derive(Copy, Clone, Debug, Pod, Zeroable)]
struct Instance {
    pos: [f32; 2],
    size: [f32; 2],
}

impl Instance {
    fn new(x: f32, y: f32, width: f32, height: f32) -> Self {
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

    uniform_buffer: wgpu::Buffer,
    staging_buffer: wgpu::Buffer,
    uniform_bind_group: wgpu::BindGroup,
}

impl<'window> Gpu<'window> {
    pub fn new(window: Arc<Window>) -> Gpu<'window> {
        pollster::block_on(Gpu::new_async(window))
    }

    pub async fn new_async(window: Arc<Window>) -> Gpu<'window> {
        let size = window.inner_size();
        let width = size.width.max(1);
        let height = size.height.max(1);

        /*
         * wgpu
         */

        let instance = wgpu::Instance::default();
        let surface = instance.create_surface(Arc::clone(&window)).unwrap();

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
                    required_features: wgpu::Features::empty(),
                    required_limits: wgpu::Limits::downlevel_webgl2_defaults()
                        .using_resolution(adapter.limits()),
                    memory_hints: Performance,
                },
                None,
            )
            .await
            .expect("Failed to create device");

        let config = surface.get_default_config(&adapter, width, height).unwrap();

        surface.configure(&device, &config);

        /*
         * rects
         */

        let rects: Vec<Instance> = vec![
            Instance::new(100.0, 100.0, 200.0, 200.0),
            Instance::new(400.0, 100.0, 200.0, 200.0),
            Instance::new(700.0, 100.0, 200.0, 200.0),
        ];

        /*
         * instances
         */

        let instance_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Instance Buffer"),
            contents: cast_slice(&rects),
            usage: wgpu::BufferUsages::VERTEX,
        });
        let instance_count = rects.len() as u32;

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
         * uniforms
         */

        let uniforms = Uniforms {
            screen_size: [width as f32, height as f32],
        };

        let uniform_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Uniform Buffer"),
            contents: cast_slice(&[uniforms]),
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
        });

        let staging_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Staging Buffer"),
            contents: cast_slice(&[uniforms]),
            usage: wgpu::BufferUsages::COPY_SRC | wgpu::BufferUsages::COPY_DST,
        });

        let uniform_bind_group_layout =
            device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
                label: Some("Uniform Bind Group Layout"),
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

        let uniform_bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("Uniform Bind Group"),
            layout: &uniform_bind_group_layout,
            entries: &[wgpu::BindGroupEntry {
                binding: 0,
                resource: uniform_buffer.as_entire_binding(),
            }],
        });

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
                bind_group_layouts: &[&uniform_bind_group_layout],
                push_constant_ranges: &[],
            });

        let render_pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: None,
            layout: Some(&render_pipeline_layout),
            vertex: wgpu::VertexState {
                module: &shader,
                entry_point: Some("vs_main"),
                buffers: &[
                    wgpu::VertexBufferLayout {
                        array_stride: std::mem::size_of::<[f32; 2]>() as wgpu::BufferAddress,
                        step_mode: wgpu::VertexStepMode::Vertex,
                        attributes: &[wgpu::VertexAttribute {
                            offset: 0,
                            shader_location: 0,
                            format: wgpu::VertexFormat::Float32x2,
                        }],
                    },
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
                // strip_index_format: None,
                // front_face: wgpu::FrontFace::Ccw,
                // cull_mode: Some(wgpu::Face::Back),
                // // Setting this to anything other than Fill requires Features::POLYGON_MODE_LINE
                // // or Features::POLYGON_MODE_POINT
                // polygon_mode: wgpu::PolygonMode::Fill,
                // // Requires Features::DEPTH_CLIP_CONTROL
                // unclipped_depth: false,
                // // Requires Features::CONSERVATIVE_RASTERIZATION
                // conservative: false,
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
            uniform_buffer,
            staging_buffer,
            uniform_bind_group,
        }
    }

    pub fn resize(&mut self, width: u32, height: u32) {
        //        let width = width.max(1);
        //        let height = height.max(1);
        //
        //        self.config.width = width;
        //        self.config.height = height;
        //
        //        // block until the GPU is idle to reduce gliting
        //        self.device.poll(wgpu::Maintain::Wait);
        //
        //        self.surface.configure(&self.device, &self.config);
        //
        //        let surface_uniform = Uniforms {
        //            screen_size: [width as f32, height as f32],
        //        };
        //
        //        self.queue
        //            .write_buffer(&self.uniform_buffer, 0, cast_slice(&[surface_uniform]));

        let width = width.max(1);
        let height = height.max(1);

        self.config.width = width;
        self.config.height = height;

        // block until the GPU is idle to reduce glitching
        self.device.poll(wgpu::Maintain::Wait);
        self.surface.configure(&self.device, &self.config);

        let new_uniform_data = [width as f32, height as f32];
        self.queue.write_buffer(
            &self.staging_buffer,
            0,
            bytemuck::cast_slice(&new_uniform_data),
        );

        // reduce glitching: wait for the GPU to finish
        let mut encoder = self
            .device
            .create_command_encoder(&wgpu::CommandEncoderDescriptor {
                label: Some("Uniform Update Encoder"),
            });

        encoder.copy_buffer_to_buffer(
            &self.staging_buffer,
            0,
            &self.uniform_buffer,
            0,
            std::mem::size_of::<[f32; 2]>() as u64,
        );

        self.queue.submit(Some(encoder.finish()));
    }

    pub fn draw(&mut self) {
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
                        load: wgpu::LoadOp::Clear(wgpu::Color {
                            r: 0.104,
                            g: 0.133,
                            b: 0.212,
                            a: 1.0,
                        }),
                        store: wgpu::StoreOp::Store,
                    },
                })],
                depth_stencil_attachment: None,
                timestamp_writes: None,
                occlusion_query_set: None,
            });

            rpass.set_pipeline(&self.render_pipeline);
            rpass.set_bind_group(0, &self.uniform_bind_group, &[]);
            rpass.set_vertex_buffer(0, self.vertex_buffer.slice(..));
            rpass.set_vertex_buffer(1, self.instance_buffer.slice(..));
            rpass.set_index_buffer(self.index_buffer.slice(..), wgpu::IndexFormat::Uint16);
            rpass.draw_indexed(0..self.num_indices, 0, 0..self.instance_count as _);
        }

        self.queue.submit(Some(encoder.finish()));
        frame.present();
    }
}
