use cosmic_text::{Attrs, Buffer, Family, FontSystem, Metrics, Shaping, SwashCache, Weight, Wrap};
use std::collections::HashMap;
use wgpu::util::DeviceExt;

pub struct TextAtlas {
    texture: wgpu::Texture,
    texture_view: wgpu::TextureView,
    width: u32,
    height: u32,
    // Maps cache key to position in atlas
    glyph_cache: HashMap<u64, GlyphInfo>, // Using u64 as simplified cache key
    // Current position for packing new glyphs
    current_x: u32,
    current_y: u32,
    row_height: u32,
}

#[derive(Clone, Copy)]
pub struct GlyphInfo {
    pub x: u32,
    pub y: u32,
    pub width: u32,
    pub height: u32,
    pub bearing_left: i32, // horizontal bearing from placement
    pub bearing_top: i32,  // vertical bearing from placement
}

impl TextAtlas {
    pub fn new(device: &wgpu::Device, width: u32, height: u32) -> Self {
        let texture = device.create_texture(&wgpu::TextureDescriptor {
            label: Some("Text Atlas"),
            size: wgpu::Extent3d {
                width,
                height,
                depth_or_array_layers: 1,
            },
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format: wgpu::TextureFormat::R8Unorm,
            usage: wgpu::TextureUsages::TEXTURE_BINDING | wgpu::TextureUsages::COPY_DST,
            view_formats: &[],
        });

        let texture_view = texture.create_view(&wgpu::TextureViewDescriptor::default());

        Self {
            texture,
            texture_view,
            width,
            height,
            glyph_cache: HashMap::new(),
            current_x: 2, // Start with padding
            current_y: 2, // Start with padding
            row_height: 0,
        }
    }

    pub fn get_or_insert_glyph(
        &mut self,
        _device: &wgpu::Device,
        queue: &wgpu::Queue,
        cache_key: u64,
        glyph_data: &[u8],
        glyph_width: u32,
        glyph_height: u32,
    ) -> Option<GlyphInfo> {
        if let Some(glyph_info) = self.glyph_cache.get(&cache_key) {
            return Some(*glyph_info);
        }

        // Add padding between glyphs to prevent texture bleeding
        let padding = 2u32; // 2-pixel padding around each glyph

        // Check if glyph fits in current row (including padding)
        if self.current_x + glyph_width + padding > self.width {
            // Move to next row
            self.current_x = padding; // Start new row with padding
            self.current_y += self.row_height + padding; // Add vertical padding too
            self.row_height = 0;
        }

        // Check if glyph fits in atlas (including padding)
        if self.current_y + glyph_height + padding > self.height {
            return None; // Atlas is full
        }

        let glyph_info = GlyphInfo {
            x: self.current_x,
            y: self.current_y,
            width: glyph_width,
            height: glyph_height,
            bearing_left: 0, // Default for fallback rectangles
            bearing_top: 0,  // Default for fallback rectangles
        };

        // Upload glyph data to texture
        queue.write_texture(
            wgpu::TexelCopyTextureInfo {
                texture: &self.texture,
                mip_level: 0,
                origin: wgpu::Origin3d {
                    x: self.current_x,
                    y: self.current_y,
                    z: 0,
                },
                aspect: wgpu::TextureAspect::All,
            },
            glyph_data,
            wgpu::TexelCopyBufferLayout {
                offset: 0,
                bytes_per_row: Some(glyph_width),
                rows_per_image: Some(glyph_height),
            },
            wgpu::Extent3d {
                width: glyph_width,
                height: glyph_height,
                depth_or_array_layers: 1,
            },
        );

        self.glyph_cache.insert(cache_key, glyph_info);

        // Update position for next glyph (add padding to prevent bleeding)
        self.current_x += glyph_width + padding;
        self.row_height = self.row_height.max(glyph_height);

        Some(glyph_info)
    }

    pub fn texture_view(&self) -> &wgpu::TextureView {
        &self.texture_view
    }
}

#[repr(C)]
#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
pub struct TextInstance {
    pub pos: [f32; 2],
    pub size: [f32; 2],
    pub tex_coords: [f32; 4], // x, y, width, height in texture coordinates
    pub color: [f32; 4],
}

impl TextInstance {
    pub fn new(
        x: f32,
        y: f32,
        width: f32,
        height: f32,
        tex_x: f32,
        tex_y: f32,
        tex_width: f32,
        tex_height: f32,
        color: [f32; 4],
    ) -> Self {
        Self {
            pos: [x, y],
            size: [width, height],
            tex_coords: [tex_x, tex_y, tex_width, tex_height],
            color,
        }
    }
}

pub struct TextRenderer {
    pub font_system: FontSystem,
    pub swash_cache: SwashCache,
    pub atlas: TextAtlas,
    pub render_pipeline: wgpu::RenderPipeline,
    pub bind_group: wgpu::BindGroup,
    pub sampler: wgpu::Sampler,
    pub instance_buffer: wgpu::Buffer,
    pub instance_count: u32,
}

impl TextRenderer {
    pub fn new(device: &wgpu::Device, _queue: &wgpu::Queue, format: wgpu::TextureFormat) -> Self {
        let font_system = FontSystem::new();
        let swash_cache = SwashCache::new();
        let atlas = TextAtlas::new(device, 1024, 1024);

        // Create sampler for text texture
        let sampler = device.create_sampler(&wgpu::SamplerDescriptor {
            address_mode_u: wgpu::AddressMode::ClampToEdge,
            address_mode_v: wgpu::AddressMode::ClampToEdge,
            address_mode_w: wgpu::AddressMode::ClampToEdge,
            mag_filter: wgpu::FilterMode::Linear,
            min_filter: wgpu::FilterMode::Linear,
            mipmap_filter: wgpu::FilterMode::Nearest,
            ..Default::default()
        });

        // Create bind group layout
        let bind_group_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            entries: &[
                wgpu::BindGroupLayoutEntry {
                    binding: 0,
                    visibility: wgpu::ShaderStages::FRAGMENT,
                    ty: wgpu::BindingType::Texture {
                        multisampled: false,
                        view_dimension: wgpu::TextureViewDimension::D2,
                        sample_type: wgpu::TextureSampleType::Float { filterable: true },
                    },
                    count: None,
                },
                wgpu::BindGroupLayoutEntry {
                    binding: 1,
                    visibility: wgpu::ShaderStages::FRAGMENT,
                    ty: wgpu::BindingType::Sampler(wgpu::SamplerBindingType::Filtering),
                    count: None,
                },
            ],
            label: Some("Text Bind Group Layout"),
        });

        // Create bind group
        let bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            layout: &bind_group_layout,
            entries: &[
                wgpu::BindGroupEntry {
                    binding: 0,
                    resource: wgpu::BindingResource::TextureView(atlas.texture_view()),
                },
                wgpu::BindGroupEntry {
                    binding: 1,
                    resource: wgpu::BindingResource::Sampler(&sampler),
                },
            ],
            label: Some("Text Bind Group"),
        });

        // Create shader
        let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("Text Shader"),
            source: wgpu::ShaderSource::Wgsl(include_str!("text_shader.wgsl").into()),
        });

        // Create pipeline layout
        let push_constant_range = wgpu::PushConstantRange {
            stages: wgpu::ShaderStages::VERTEX,
            range: 0..std::mem::size_of::<[f32; 2]>() as u32,
        };

        let pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: Some("Text Pipeline Layout"),
            bind_group_layouts: &[&bind_group_layout],
            push_constant_ranges: &[push_constant_range],
        });

        // Create render pipeline
        let render_pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("Text Render Pipeline"),
            layout: Some(&pipeline_layout),
            vertex: wgpu::VertexState {
                module: &shader,
                entry_point: Some("vs_main"),
                buffers: &[wgpu::VertexBufferLayout {
                    array_stride: std::mem::size_of::<TextInstance>() as wgpu::BufferAddress,
                    step_mode: wgpu::VertexStepMode::Instance,
                    attributes: &[
                        // pos
                        wgpu::VertexAttribute {
                            offset: 0,
                            shader_location: 0,
                            format: wgpu::VertexFormat::Float32x2,
                        },
                        // size
                        wgpu::VertexAttribute {
                            offset: std::mem::size_of::<[f32; 2]>() as wgpu::BufferAddress,
                            shader_location: 1,
                            format: wgpu::VertexFormat::Float32x2,
                        },
                        // tex_coords
                        wgpu::VertexAttribute {
                            offset: std::mem::size_of::<[f32; 4]>() as wgpu::BufferAddress,
                            shader_location: 2,
                            format: wgpu::VertexFormat::Float32x4,
                        },
                        // color
                        wgpu::VertexAttribute {
                            offset: std::mem::size_of::<[f32; 8]>() as wgpu::BufferAddress,
                            shader_location: 3,
                            format: wgpu::VertexFormat::Float32x4,
                        },
                    ],
                }],
                compilation_options: Default::default(),
            },
            fragment: Some(wgpu::FragmentState {
                module: &shader,
                entry_point: Some("fs_main"),
                compilation_options: Default::default(),
                targets: &[Some(wgpu::ColorTargetState {
                    format,
                    blend: Some(wgpu::BlendState::ALPHA_BLENDING),
                    write_mask: wgpu::ColorWrites::ALL,
                })],
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

        // Create initial empty instance buffer
        let instance_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Text Instance Buffer"),
            contents: &[],
            usage: wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::COPY_DST,
        });

        Self {
            font_system,
            swash_cache,
            atlas,
            render_pipeline,
            bind_group,
            sampler,
            instance_buffer,
            instance_count: 0,
        }
    }

    pub fn render_text(
        &mut self,
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        text: &str,
        x: f32,
        y: f32,
        font_size: f32,
        color: [f32; 4],
        max_width: Option<f32>,
    ) -> Vec<TextInstance> {
        println!(
            "render_text called: '{}' at ({}, {}) size {} color {:?}",
            text, x, y, font_size, color
        );

        let mut buffer = Buffer::new(&mut self.font_system, Metrics::new(font_size, font_size));

        let wrap = if max_width.is_some() {
            Wrap::Word
        } else {
            Wrap::None
        };

        buffer.set_size(&mut self.font_system, max_width, None);
        buffer.set_text(
            &mut self.font_system,
            text,
            Attrs::new()
                .family(Family::SansSerif)
                .weight(Weight::NORMAL),
            Shaping::Advanced,
        );
        buffer.set_wrap(&mut self.font_system, wrap);
        buffer.shape_until_scroll(&mut self.font_system, false);

        // Ensure we have at least a white pixel in the atlas for fallback rendering
        self.ensure_white_pixel(device, queue);

        let mut instances = Vec::new();

        for run in buffer.layout_runs() {
            println!("Layout run with {} glyphs", run.glyphs.len());
            for glyph in run.glyphs.iter() {
                let glyph_width = glyph.w;

                println!(
                    "Glyph: id={} x={} y={} w={}",
                    glyph.glyph_id, glyph.x, glyph.y, glyph_width
                );

                if glyph_width > 0.0 {
                    // Generate a simple cache key for this specific glyph (glyph_id + font_size)
                    let cache_key = ((glyph.glyph_id as u64) << 32) | (font_size as u64);

                    // Try to get the glyph from cache, or render it if not cached
                    let glyph_info =
                        if let Some(cached_glyph) = self.atlas.glyph_cache.get(&cache_key) {
                            *cached_glyph
                        } else {
                            // Render the glyph using cosmic_text
                            println!("Rendering glyph {} to texture atlas", glyph.glyph_id);

                            // Create cache key for this glyph
                            let (swash_cache_key, _, _) = cosmic_text::CacheKey::new(
                                glyph.font_id,
                                glyph.glyph_id,
                                glyph.font_size,
                                (0.0, 0.0),                          // subpixel offset
                                cosmic_text::CacheKeyFlags::empty(), // flags
                            );

                            // Get glyph image using the swash cache
                            let image_option = self
                                .swash_cache
                                .get_image(&mut self.font_system, swash_cache_key);

                            if let Some(image) = image_option {
                                println!(
                                "Got glyph image: {}x{} format {:?}, placement: left={}, top={}",
                                image.placement.width, image.placement.height, image.content,
                                image.placement.left, image.placement.top
                            );

                                // Convert the image data to R8 format (alpha only)
                                let alpha_data = match image.content {
                                    cosmic_text::SwashContent::Mask => {
                                        // Mask data is already single-channel alpha
                                        image.data.to_vec()
                                    }
                                    cosmic_text::SwashContent::Color => {
                                        // Color data is BGRA, extract alpha channel
                                        image.data.chunks(4).map(|bgra| bgra[3]).collect()
                                    }
                                    cosmic_text::SwashContent::SubpixelMask => {
                                        // SubpixelMask is RGB, convert to grayscale
                                        image
                                            .data
                                            .chunks(3)
                                            .map(|rgb| {
                                                (rgb[0] as f32 * 0.299
                                                    + rgb[1] as f32 * 0.587
                                                    + rgb[2] as f32 * 0.114)
                                                    as u8
                                            })
                                            .collect()
                                    }
                                };

                                println!("Converted glyph to {} alpha bytes", alpha_data.len());

                                // Add to atlas
                                if let Some(atlas_info) = self.atlas.get_or_insert_glyph(
                                    device,
                                    queue,
                                    cache_key,
                                    &alpha_data,
                                    image.placement.width,
                                    image.placement.height,
                                ) {
                                    println!(
                                        "Added glyph to atlas at ({}, {})",
                                        atlas_info.x, atlas_info.y
                                    );
                                    // Store the glyph info with bearing information from placement
                                    let glyph_info_with_bearing = GlyphInfo {
                                        x: atlas_info.x,
                                        y: atlas_info.y,
                                        width: atlas_info.width,
                                        height: atlas_info.height,
                                        bearing_left: image.placement.left,
                                        bearing_top: image.placement.top,
                                    };
                                    self.atlas
                                        .glyph_cache
                                        .insert(cache_key, glyph_info_with_bearing);
                                    glyph_info_with_bearing
                                } else {
                                    println!(
                                    "Failed to add glyph to atlas, using white rectangle fallback"
                                );
                                    // Fallback to white rectangle
                                    GlyphInfo {
                                        x: 0,
                                        y: 0,
                                        width: 8,
                                        height: 8,
                                        bearing_left: 0,
                                        bearing_top: 0,
                                    }
                                }
                            } else {
                                println!(
                                    "Failed to render glyph {}, using white rectangle fallback",
                                    glyph.glyph_id
                                );
                                // Fallback to white rectangle for space characters or missing glyphs
                                GlyphInfo {
                                    x: 0,
                                    y: 0,
                                    width: 8,
                                    height: 8,
                                    bearing_left: 0,
                                    bearing_top: 0,
                                }
                            }
                        };

                    // Create text instance with proper texture coordinates
                    // No artificial padding needed since glyphs are spaced apart in atlas
                    let tex_x = glyph_info.x as f32 / self.atlas.width as f32;
                    let tex_y = glyph_info.y as f32 / self.atlas.height as f32;
                    let tex_width = glyph_info.width as f32 / self.atlas.width as f32;
                    let tex_height = glyph_info.height as f32 / self.atlas.height as f32;

                    // Use the actual glyph bitmap dimensions for rendering (not advance width)
                    let render_width = glyph_info.width as f32;
                    let render_height = glyph_info.height as f32;

                    // Calculate proper baseline-aligned position using bearing information
                    // bearing_left: horizontal offset from logical position to bitmap left edge
                    // bearing_top: distance from baseline to bitmap top edge (subtract to position correctly)
                    let render_x = x + glyph.x + glyph_info.bearing_left as f32;
                    let render_y = y + glyph.y - glyph_info.bearing_top as f32;

                    let instance = TextInstance::new(
                        render_x,
                        render_y,
                        render_width,
                        render_height,
                        tex_x,
                        tex_y,
                        tex_width,
                        tex_height,
                        color,
                    );

                    instances.push(instance);
                    println!("Created text instance at ({}, {}) size ({}, {}) with tex coords ({}, {}, {}, {}) bearing ({}, {})", 
                             render_x, render_y, render_width, render_height,
                             tex_x, tex_y, tex_width, tex_height,
                             glyph_info.bearing_left, glyph_info.bearing_top);
                }
            }
        }

        println!("render_text created {} instances", instances.len());
        instances
    }

    // Helper method to ensure we have a white pixel in the atlas for fallback rendering
    fn ensure_white_pixel(&mut self, device: &wgpu::Device, queue: &wgpu::Queue) {
        println!("ensure_white_pixel called");

        // Check if we already have a white rectangle at key 0
        if !self.atlas.glyph_cache.contains_key(&0) {
            println!("Creating white rectangle in atlas");
            // Create a 8x8 white rectangle for better sampling
            let white_rect_data = vec![255u8; 8 * 8]; // 8x8 white rectangle for R8Unorm format
            if let Some(glyph_info) =
                self.atlas
                    .get_or_insert_glyph(device, queue, 0, &white_rect_data, 8, 8)
            {
                println!(
                    "White rectangle created at ({}, {}) size {}x{} in atlas",
                    glyph_info.x, glyph_info.y, glyph_info.width, glyph_info.height
                );
            } else {
                println!("Failed to create white rectangle in atlas");
            }
        } else {
            println!("White rectangle already exists in atlas");
        }
    }

    pub fn update_instances(
        &mut self,
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        instances: &[TextInstance],
    ) {
        println!("update_instances called with {} instances", instances.len());

        if instances.is_empty() {
            self.instance_count = 0;
            return;
        }

        let contents = bytemuck::cast_slice(instances);

        // Create new buffer if needed or if the current buffer is too small
        if contents.len() > self.instance_buffer.size() as usize {
            self.instance_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
                label: Some("Text Instance Buffer"),
                contents,
                usage: wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::COPY_DST,
            });
        } else {
            // Update existing buffer
            queue.write_buffer(&self.instance_buffer, 0, contents);
        }

        self.instance_count = instances.len() as u32;
        println!(
            "Text instance buffer updated with {} instances",
            self.instance_count
        );
    }

    pub fn draw(&self, render_pass: &mut wgpu::RenderPass, viewport: [f32; 2]) {
        println!(
            "Text draw called with {} instances, viewport: {:?}",
            self.instance_count, viewport
        );

        if self.instance_count == 0 {
            println!("No text instances to draw");
            return;
        }

        println!("Setting up text render pass...");
        render_pass.set_pipeline(&self.render_pipeline);
        render_pass.set_bind_group(0, &self.bind_group, &[]);
        render_pass.set_push_constants(
            wgpu::ShaderStages::VERTEX,
            0,
            bytemuck::bytes_of(&viewport),
        );
        render_pass.set_vertex_buffer(0, self.instance_buffer.slice(..));

        println!(
            "Drawing {} text instances with 6 vertices each",
            self.instance_count
        );
        render_pass.draw(0..6, 0..self.instance_count);

        println!(
            "Text draw executed successfully for {} instances",
            self.instance_count
        );
    }
}
