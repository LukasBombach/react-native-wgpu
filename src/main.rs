use deno_core::*;
use serde::Deserialize;
use std::borrow::Cow;
use std::sync::{Arc, Mutex};
use winit::{
    event::*,
    event_loop::EventLoop,
    window::{Window, WindowBuilder},
};

// ===================
// Deno Core & Global State
// ===================

#[derive(Debug, Clone)]
struct Triangle {
    size: f32,
    position: (f32, f32),
    color: [f32; 4],
}

#[derive(Default)]
struct AppState {
    triangles: Vec<Triangle>,
}

#[derive(Debug, Deserialize)]
struct TriangleInput {
    size: f32,
    position: [f32; 2],
    color: [f32; 4],
}

/// Op, der ein Dreieck erstellt – Eingabe wird automatisch via Serde deserialisiert.
#[op2]
fn op_create_triangle(
    state: &mut OpState,
    #[serde] input: TriangleInput,
) -> Result<(), deno_error::JsErrorBox> {
    let app_state = state.borrow::<Arc<Mutex<AppState>>>().clone();
    {
        let mut app_state = app_state.lock().unwrap();
        app_state.triangles.push(Triangle {
            size: input.size,
            position: (input.position[0], input.position[1]),
            color: input.color,
        });
    }
    Ok(())
}

// ===================
// Renderer mit wgpu
// ===================

use wgpu::util::DeviceExt;

#[repr(C)]
#[derive(Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
struct Vertex {
    position: [f32; 2],
}
impl Vertex {
    const ATTRIBS: [wgpu::VertexAttribute; 1] = wgpu::vertex_attr_array![0 => Float32x2];
    fn desc<'a>() -> wgpu::VertexBufferLayout<'a> {
        wgpu::VertexBufferLayout {
            array_stride: std::mem::size_of::<Vertex>() as wgpu::BufferAddress,
            step_mode: wgpu::VertexStepMode::Vertex,
            attributes: &Self::ATTRIBS,
        }
    }
}
const VERTICES: &[Vertex] = &[
    Vertex {
        position: [-0.5, -0.5],
    },
    Vertex {
        position: [0.5, -0.5],
    },
    Vertex {
        position: [0.0, 0.5],
    },
];

#[repr(C)]
#[derive(Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
struct Instance {
    position: [f32; 2],
    size: f32,
    _padding: f32, // Padding, damit die Struktur 16-Byte-aligned ist
    color: [f32; 4],
}
impl Instance {
    const ATTRIBS: [wgpu::VertexAttribute; 3] = wgpu::vertex_attr_array![
        1 => Float32x2, // instance position
        2 => Float32,   // instance size
        3 => Float32x4, // instance color
    ];
    fn desc<'a>() -> wgpu::VertexBufferLayout<'a> {
        wgpu::VertexBufferLayout {
            array_stride: std::mem::size_of::<Instance>() as wgpu::BufferAddress,
            step_mode: wgpu::VertexStepMode::Instance,
            attributes: &Self::ATTRIBS,
        }
    }
}

// Uniform für die Transformation: Hier speichern wir die halbe Fenstergröße,
// um Pixelkoordinaten in den NDC-Raum ([–1,1]) umzurechnen.
#[repr(C)]
#[derive(Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
struct Transform {
    half_size: [f32; 2],
}

// WGSL-Shader: Vertex-Shader berechnet die Endposition aus
// Basis-Dreieck und Instanz-Daten.
const VERT_SHADER: &str = r#"
struct VertexOutput {
    @builtin(position) position: vec4<f32>,
    @location(0) color: vec4<f32>,
};

@group(0) @binding(0)
var<uniform> u_transform: vec2<f32>;

@vertex
fn main(
    @location(0) vertex_pos: vec2<f32>,
    @location(1) instance_pos: vec2<f32>,
    @location(2) instance_size: f32,
    @location(3) instance_color: vec4<f32>
) -> VertexOutput {
    var out: VertexOutput;
    let scaled = vertex_pos * instance_size;
    // Umrechnung von Pixelkoordinaten in NDC:
    let pos = (scaled + instance_pos) / u_transform - vec2<f32>(1.0, 1.0);
    out.position = vec4<f32>(pos, 0.0, 1.0);
    out.color = instance_color;
    return out;
}
"#;

const FRAG_SHADER: &str = r#"
@fragment
fn main(@location(0) color: vec4<f32>) -> @location(0) vec4<f32> {
    return color;
}
"#;

struct Renderer<'a> {
    surface: wgpu::Surface<'a>,
    device: wgpu::Device,
    queue: wgpu::Queue,
    config: wgpu::SurfaceConfiguration,
    render_pipeline: wgpu::RenderPipeline,
    vertex_buffer: wgpu::Buffer,
    instance_buffer: wgpu::Buffer,
    instance_capacity: usize,
    uniform_buffer: wgpu::Buffer,
    uniform_bind_group: wgpu::BindGroup,
    window: Arc<Window>,
}

impl Renderer<'_> {
    async fn new(window: Window) -> Self {
        let window = Arc::new(window);
        let size = window.inner_size();
        let instance_wgpu = wgpu::Instance::new(&wgpu::InstanceDescriptor {
            backends: wgpu::Backends::all(),
            ..Default::default()
        });
        let surface = instance_wgpu.create_surface(window.clone()).unwrap();
        let adapter = instance_wgpu
            .request_adapter(&wgpu::RequestAdapterOptions {
                power_preference: wgpu::PowerPreference::HighPerformance,
                compatible_surface: Some(&surface),
                force_fallback_adapter: false,
            })
            .await
            .unwrap();
        let (device, queue) = adapter
            .request_device(
                &wgpu::DeviceDescriptor {
                    label: None,
                    required_features: wgpu::Features::empty(),
                    required_limits: wgpu::Limits::downlevel_webgl2_defaults()
                        .using_resolution(adapter.limits()),
                    memory_hints: wgpu::MemoryHints::MemoryUsage,
                },
                None,
            )
            .await
            .unwrap();

        let caps = surface.get_capabilities(&adapter);
        let format = caps.formats[0];
        let config = wgpu::SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format,
            width: size.width,
            height: size.height,
            present_mode: wgpu::PresentMode::Fifo,
            alpha_mode: wgpu::CompositeAlphaMode::Auto,
            view_formats: vec![],
            desired_maximum_frame_latency: 1,
        };
        surface.configure(&device, &config);

        let shader_module = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("Vertex Shader"),
            source: wgpu::ShaderSource::Wgsl(VERT_SHADER.into()),
        });
        let frag_module = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("Fragment Shader"),
            source: wgpu::ShaderSource::Wgsl(FRAG_SHADER.into()),
        });

        // Uniform für Transform: Halbe Fenstergröße in Pixeln
        let transform = Transform {
            half_size: [size.width as f32 / 2.0, size.height as f32 / 2.0],
        };
        let uniform_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Uniform Buffer"),
            contents: bytemuck::bytes_of(&transform),
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
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

        let pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: Some("Render Pipeline Layout"),
            bind_group_layouts: &[&uniform_bind_group_layout],
            push_constant_ranges: &[],
        });

        let render_pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            cache: None,
            label: Some("Render Pipeline"),
            layout: Some(&pipeline_layout),
            vertex: wgpu::VertexState {
                module: &shader_module,
                entry_point: Some("main"),
                buffers: &[Vertex::desc(), Instance::desc()],
                compilation_options: wgpu::PipelineCompilationOptions::default(),
            },
            fragment: Some(wgpu::FragmentState {
                module: &frag_module,
                entry_point: Some("main"),
                compilation_options: wgpu::PipelineCompilationOptions::default(),
                targets: &[Some(wgpu::ColorTargetState {
                    format,
                    blend: Some(wgpu::BlendState::ALPHA_BLENDING),
                    write_mask: wgpu::ColorWrites::ALL,
                })],
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
        });

        let vertex_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Vertex Buffer"),
            contents: bytemuck::cast_slice(VERTICES),
            usage: wgpu::BufferUsages::VERTEX,
        });

        // Erzeuge einen leeren Instanzpuffer mit Kapazität für z. B. 100 Instanzen.
        let instance_capacity = 100;
        let instance_buffer = device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("Instance Buffer"),
            size: (instance_capacity * std::mem::size_of::<Instance>()) as wgpu::BufferAddress,
            usage: wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });

        Self {
            surface,
            device,
            queue,
            config,
            render_pipeline,
            vertex_buffer,
            instance_buffer,
            instance_capacity,
            uniform_buffer,
            uniform_bind_group,
            window,
        }
    }

    fn resize(&mut self, new_size: winit::dpi::PhysicalSize<u32>) {
        self.config.width = new_size.width;
        self.config.height = new_size.height;
        self.surface.configure(&self.device, &self.config);
        let transform = Transform {
            half_size: [new_size.width as f32 / 2.0, new_size.height as f32 / 2.0],
        };
        self.queue
            .write_buffer(&self.uniform_buffer, 0, bytemuck::bytes_of(&transform));
    }

    /// Aktualisiert den Instanzpuffer anhand der aktuellen Instanzdaten.
    fn update(&mut self, instances: &[Instance]) {
        let instance_data = bytemuck::cast_slice(instances);
        self.queue
            .write_buffer(&self.instance_buffer, 0, instance_data);
    }

    fn render(&mut self) -> Result<(), wgpu::SurfaceError> {
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
                occlusion_query_set: None,
                timestamp_writes: None,
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: &view,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(wgpu::Color::BLACK),
                        store: wgpu::StoreOp::Store,
                    },
                })],
                depth_stencil_attachment: None,
            });
            render_pass.set_pipeline(&self.render_pipeline);
            render_pass.set_bind_group(0, &self.uniform_bind_group, &[]);
            render_pass.set_vertex_buffer(0, self.vertex_buffer.slice(..));
            render_pass.set_vertex_buffer(1, self.instance_buffer.slice(..));
            // Zeichnet alle Instanzen – hier nutzen wir als Instanzanzahl die Kapazität (kann auch dynamisch anhand der tatsächlichen Anzahl gesetzt werden)
            render_pass.draw(0..VERTICES.len() as u32, 0..self.instance_capacity as u32);
        }

        self.queue.submit(std::iter::once(encoder.finish()));
        output.present();
        Ok(())
    }
}

// ===================
// Integration: Deno Runtime + Renderer
// ===================

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Globalen Zustand initialisieren und in die Deno-Runtime einfügen.
    let app_state = Arc::new(Mutex::new(AppState::default()));

    // Erstelle die Deno-Extension.
    const DECL: OpDecl = op_create_triangle();
    let ext = Extension {
        name: "triangle_ext",
        ops: Cow::Borrowed(&[DECL]),
        ..Default::default()
    };

    let mut runtime = JsRuntime::new(RuntimeOptions {
        extensions: vec![ext],
        ..Default::default()
    });
    runtime.op_state().borrow_mut().put(app_state.clone());

    // Führe JavaScript-Code aus, der zwei Dreiecke erstellt.
    runtime
        .execute_script(
            "<init>",
            r#"
        class Triangle {
            constructor(size, position, color) {
                Deno.core.ops.op_create_triangle({ size, position, color });
            }
        }
        new Triangle(100, [400, 300], [1.0, 0.0, 0.0, 1.0]);
        new Triangle(80, [600, 300], [0.0, 1.0, 0.0, 1.0]);
        Deno.core.print("Triangles created\n");
        "#,
        )
        .unwrap();

    // Starte den winit-EventLoop und initialisiere den Renderer.
    let event_loop = EventLoop::new().unwrap();
    let window = WindowBuilder::new()
        .with_title("Triangle Renderer")
        .with_inner_size(winit::dpi::PhysicalSize::new(800, 600))
        .build(&event_loop)
        .unwrap();

    let mut renderer = Renderer::new(window).await;
    let app_state_render = app_state.clone();

    event_loop
        .run(move |event, target| {
            // Have the closure take ownership of the resources.
            // `event_loop.run` never returns, therefore we must do this to ensure
            // the resources are properly cleaned up.

            //let _ = (&instance, &adapter, &shader, &pipeline_layout);

            if let Event::WindowEvent {
                window_id: _,
                event,
            } = event
            {
                match event {
                    WindowEvent::CloseRequested => target.exit(),
                    WindowEvent::Resized(physical_size) => renderer.resize(physical_size),
                    WindowEvent::RedrawRequested => {
                        // Hole die aktuellen Dreiecke aus dem globalen Zustand
                        let triangles = {
                            let state = app_state_render.lock().unwrap();
                            state.triangles.clone()
                        };
                        // Wandele sie in Instanz-Daten um
                        let instances: Vec<Instance> = triangles
                            .iter()
                            .map(|tri| Instance {
                                position: [tri.position.0, tri.position.1],
                                size: tri.size,
                                _padding: 0.0,
                                color: tri.color,
                            })
                            .collect();
                        renderer.update(&instances);
                        match renderer.render() {
                            Ok(_) => {}
                            Err(wgpu::SurfaceError::Lost) => {
                                // renderer.resize(renderer.window.inner_size())
                            }
                            Err(wgpu::SurfaceError::OutOfMemory) => target.exit(),
                            Err(e) => eprintln!("{:?}", e),
                        }
                    }

                    // Event::MainEventsCleared => {
                    //     window.request_redraw();
                    // }
                    _ => {}
                }
            }
        })
        .unwrap();
    Ok(())
}
