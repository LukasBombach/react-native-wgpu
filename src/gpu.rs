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

use crate::text::TextRenderer;

#[repr(C)]
#[derive(Copy, Clone, Debug, Pod, Zeroable)]
pub struct Instance {
    pos: [f32; 2],
    size: [f32; 2],
    background_color: [f32; 4],
    border_radius: f32,
}

impl Instance {
    pub fn new(
        x: f32,
        y: f32,
        width: f32,
        height: f32,
        background_color: [f32; 4],
        border_radius: f32,
    ) -> Self {
        Self {
            pos: [x, y],
            size: [width, height],
            background_color,
            border_radius,
        }
    }
}

pub struct Gpu<'window> {
    surface: wgpu::Surface<'window>,
    config: wgpu::SurfaceConfiguration,
    device: wgpu::Device,
    queue: wgpu::Queue,
    render_pipeline: wgpu::RenderPipeline,
    instance_buffer: wgpu::Buffer,
    instance_count: u32,
    viewport: [f32; 2],
    text_renderer: TextRenderer,
}

impl<'window> Gpu<'window> {
    pub fn new(window: Arc<Window>) -> Gpu<'window> {
        pollster::block_on(Gpu::new_async(window))
    }

    pub async fn new_async(window: Arc<Window>) -> Gpu<'window> {
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

        /*
         * Jitter when resizing windows on macOS
         *
         * https://github.com/gfx-rs/wgpu/issues/3756
         * https://github.com/gfx-rs/wgpu/pull/6107
         * https://thume.ca/2019/06/19/glitchless-metal-window-resizing/
         * https://raphlinus.github.io/rust/gui/2019/06/21/smooth-resize-test.html
         */

        #[allow(invalid_reference_casting)]
        unsafe {
            surface.as_hal::<wgpu::hal::metal::Api, _, ()>(|surface| {
                if let Some(surface_ref) = surface {
                    let surface_mut = &mut *(surface_ref as *const wgpu::hal::metal::Surface
                        as *mut wgpu::hal::metal::Surface);
                    surface_mut.present_with_transaction = true;
                }
            });
        }

        /*
         * adapter
         */

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

        let mut config = surface.get_default_config(&adapter, width, height).unwrap();
        config.alpha_mode = wgpu::CompositeAlphaMode::PostMultiplied;

        surface.configure(&device, &config);

        /*
         * push constants
         */

        let push_constant_range = wgpu::PushConstantRange {
            stages: wgpu::ShaderStages::VERTEX,
            range: 0..push_const_size,
        };

        /*
         * instances
         */

        let instances: Vec<Instance> = Vec::new();

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
                buffers: &[wgpu::VertexBufferLayout {
                    array_stride: std::mem::size_of::<Instance>() as wgpu::BufferAddress,
                    step_mode: wgpu::VertexStepMode::Instance,
                    attributes: &[
                        wgpu::VertexAttribute {
                            offset: 0,
                            shader_location: 0,
                            format: wgpu::VertexFormat::Float32x2,
                        },
                        wgpu::VertexAttribute {
                            offset: std::mem::size_of::<[f32; 2]>() as wgpu::BufferAddress,
                            shader_location: 1,
                            format: wgpu::VertexFormat::Float32x2,
                        },
                        wgpu::VertexAttribute {
                            offset: std::mem::size_of::<[f32; 4]>() as wgpu::BufferAddress,
                            shader_location: 2,
                            format: wgpu::VertexFormat::Float32x4,
                        },
                        wgpu::VertexAttribute {
                            offset: std::mem::size_of::<[f32; 8]>() as wgpu::BufferAddress,
                            shader_location: 3,
                            format: wgpu::VertexFormat::Float32,
                        },
                    ],
                }],
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

        /*
         * text renderer
         */

        let text_renderer = TextRenderer::new(&device, &queue, config.format);

        Gpu {
            surface,
            config,
            device,
            queue,
            render_pipeline,
            instance_buffer,
            instance_count,
            viewport,
            text_renderer,
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

        if self.instance_count == 0 {
            return;
        }

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
                        load: wgpu::LoadOp::Clear(wgpu::Color::TRANSPARENT),
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
                rpass.set_vertex_buffer(0, self.instance_buffer.slice(..));
                rpass.draw(0..6, 0..self.instance_count);
            }

            // Render text
            self.text_renderer.draw(&mut rpass, self.viewport);
        }

        self.queue.submit(Some(encoder.finish()));
        frame.present();
    }

    pub fn update_instance_buffer(&mut self, instances: Vec<Instance>) {
        self.instance_buffer = self
            .device
            .create_buffer_init(&wgpu::util::BufferInitDescriptor {
                label: Some("Instance Buffer"),
                contents: cast_slice(&instances),
                usage: wgpu::BufferUsages::VERTEX,
            });
        self.instance_count = instances.len() as u32;
    }

    pub fn render_text(
        &mut self,
        text: &str,
        x: f32,
        y: f32,
        font_size: f32,
        color: [f32; 4],
        max_width: Option<f32>,
    ) -> Vec<crate::text::TextInstance> {
        self.text_renderer.render_text(
            &self.device,
            &self.queue,
            text,
            x,
            y,
            font_size,
            color,
            max_width,
        )
    }

    pub fn update_text_instances(&mut self, instances: &[crate::text::TextInstance]) {
        println!(
            "GPU::update_text_instances called with {} instances",
            instances.len()
        );
        self.text_renderer
            .update_instances(&self.device, &self.queue, instances);
    }
}
