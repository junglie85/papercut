use wgpu::{util::DeviceExt, RenderPass};
use winit::window::Window;

use crate::texture::Texture;

#[repr(C)]
#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
struct Vertex {
    position: [f32; 3],
    tex_coords: [f32; 2],
    color: [f32; 3],
}

impl Vertex {
    fn desc<'a>() -> wgpu::VertexBufferLayout<'a> {
        wgpu::VertexBufferLayout {
            array_stride: std::mem::size_of::<Vertex>() as wgpu::BufferAddress,
            step_mode: wgpu::VertexStepMode::Vertex,
            attributes: &[
                wgpu::VertexAttribute {
                    offset: 0,
                    shader_location: 0,
                    format: wgpu::VertexFormat::Float32x3,
                },
                wgpu::VertexAttribute {
                    offset: std::mem::size_of::<[f32; 3]>() as wgpu::BufferAddress,
                    shader_location: 1,
                    format: wgpu::VertexFormat::Float32x2,
                },
                wgpu::VertexAttribute {
                    offset: std::mem::size_of::<[f32; 5]>() as wgpu::BufferAddress,
                    shader_location: 2,
                    format: wgpu::VertexFormat::Float32x3,
                },
            ],
        }
    }
}

const SHAPE_VERTICES: &[Vertex] = &[
    Vertex {
        position: [-0.0868241, 0.49240386, 0.0],
        tex_coords: [0.0, 0.0],
        color: [0.5, 0.0, 0.5],
    }, // A
    Vertex {
        position: [-0.49513406, 0.06958647, 0.0],
        tex_coords: [0.0, 0.0],
        color: [0.5, 0.0, 0.5],
    }, // B
    Vertex {
        position: [-0.21918549, -0.44939706, 0.0],
        tex_coords: [0.0, 0.0],
        color: [0.5, 0.0, 0.5],
    }, // C
    Vertex {
        position: [0.35966998, -0.3473291, 0.0],
        tex_coords: [0.0, 0.0],
        color: [0.5, 0.0, 0.5],
    }, // D
    Vertex {
        position: [0.44147372, 0.2347359, 0.0],
        tex_coords: [0.0, 0.0],
        color: [0.5, 0.0, 0.5],
    }, // E
];

const SHAPE_INDICES: &[u16] = &[0, 1, 4, 1, 2, 4, 2, 3, 4, 0];

const SPRITE_VERTICES: &[Vertex] = &[
    Vertex {
        position: [-0.25, 0.25, 0.0],
        tex_coords: [0.0, 0.0],
        color: [1.0, 1.0, 1.0],
    }, // A
    Vertex {
        position: [-0.25, -0.75, 0.0],
        tex_coords: [0.0, 1.0],
        color: [1.0, 1.0, 1.0],
    }, // B
    Vertex {
        position: [0.75, 0.25, 0.0],
        tex_coords: [1.0, 0.0],
        color: [1.0, 1.0, 1.0],
    }, // C
    Vertex {
        position: [0.75, -0.75, 0.0],
        tex_coords: [1.0, 1.0],
        color: [1.0, 1.0, 1.0],
    }, // D
];

const SPRITE_INDICES: &[u16] = &[0, 1, 2, 2, 1, 3];

// // TODO: Combine these vertex structs
// #[repr(C)]
// #[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
// struct ShapeVertex {
//     position: [f32; 3],
//     color: [f32; 3],
// }

// // TODO: Combine these vertex structs
// #[repr(C)]
// #[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
// struct SpriteVertex {
//     position: [f32; 3],
//     tex_coords: [f32; 2],
// }

// impl ShapeVertex {
//     fn desc<'a>() -> wgpu::VertexBufferLayout<'a> {
//         wgpu::VertexBufferLayout {
//             array_stride: std::mem::size_of::<ShapeVertex>() as wgpu::BufferAddress,
//             step_mode: wgpu::VertexStepMode::Vertex,
//             attributes: &[
//                 wgpu::VertexAttribute {
//                     offset: 0,
//                     shader_location: 0,
//                     format: wgpu::VertexFormat::Float32x3,
//                 },
//                 wgpu::VertexAttribute {
//                     offset: std::mem::size_of::<[f32; 3]>() as wgpu::BufferAddress,
//                     shader_location: 1,
//                     format: wgpu::VertexFormat::Float32x3,
//                 },
//             ],
//         }
//     }
// }

// impl SpriteVertex {
//     fn desc<'a>() -> wgpu::VertexBufferLayout<'a> {
//         wgpu::VertexBufferLayout {
//             array_stride: std::mem::size_of::<SpriteVertex>() as wgpu::BufferAddress,
//             step_mode: wgpu::VertexStepMode::Vertex,
//             attributes: &[
//                 wgpu::VertexAttribute {
//                     offset: 0,
//                     shader_location: 0,
//                     format: wgpu::VertexFormat::Float32x3,
//                 },
//                 wgpu::VertexAttribute {
//                     offset: std::mem::size_of::<[f32; 3]>() as wgpu::BufferAddress,
//                     shader_location: 1,
//                     format: wgpu::VertexFormat::Float32x2,
//                 },
//             ],
//         }
//     }
// }

// const SHAPE_VERTICES: &[ShapeVertex] = &[
//     ShapeVertex {
//         position: [-0.0868241, 0.49240386, 0.0],
//         color: [0.5, 0.0, 0.5],
//     }, // A
//     ShapeVertex {
//         position: [-0.49513406, 0.06958647, 0.0],
//         color: [0.5, 0.0, 0.5],
//     }, // B
//     ShapeVertex {
//         position: [-0.21918549, -0.44939706, 0.0],
//         color: [0.5, 0.0, 0.5],
//     }, // C
//     ShapeVertex {
//         position: [0.35966998, -0.3473291, 0.0],
//         color: [0.5, 0.0, 0.5],
//     }, // D
//     ShapeVertex {
//         position: [0.44147372, 0.2347359, 0.0],
//         color: [0.5, 0.0, 0.5],
//     }, // E
// ];

// const SHAPE_INDICES: &[u16] = &[0, 1, 4, 1, 2, 4, 2, 3, 4, 0];

// const SPRITE_VERTICES: &[SpriteVertex] = &[
//     SpriteVertex {
//         position: [-0.25, 0.25, 0.0],
//         tex_coords: [0.0, 0.0],
//     }, // A
//     SpriteVertex {
//         position: [-0.25, -0.75, 0.0],
//         tex_coords: [0.0, 1.0],
//     }, // B
//     SpriteVertex {
//         position: [0.75, 0.25, 0.0],
//         tex_coords: [1.0, 0.0],
//     }, // C
//     SpriteVertex {
//         position: [0.75, -0.75, 0.0],
//         tex_coords: [1.0, 1.0],
//     }, // D
// ];

// const SPRITE_INDICES: &[u16] = &[0, 1, 2, 2, 1, 3];

#[derive(Debug)]
pub struct Renderer {
    shape_vertex_buffer: wgpu::Buffer,
    shape_index_buffer: wgpu::Buffer,
    // uniform_buffer: wgpu::Buffer,
    // bind_group: wgpu::BindGroup,
    shape_pipeline: wgpu::RenderPipeline, // TODO: This is for a (filled?) shape only! What if I want to draw a bounding box?
    // TODO: What about wireframe / outlines?
    pub(crate) clear_color: wgpu::Color,
    // width: f32,
    // height: f32,
    shape_num_indices: u32,
    /////////// Texture pipeline //////////////
    sprite_vertex_buffer: wgpu::Buffer,
    sprite_index_buffer: wgpu::Buffer,
    // uniform_buffer: wgpu::Buffer,
    // bind_group: wgpu::BindGroup,
    sprite_pipeline: wgpu::RenderPipeline,
    sprite_num_indices: u32,
    sprite_bind_group_layout: wgpu::BindGroupLayout,
}

impl Renderer {
    pub fn new(
        device: &wgpu::Device,
        // texture_view: &wgpu::TextureView,
        // texture_size: &wgpu::Extent3d,
        // surface_size: &SurfaceSize,
        surface_format: wgpu::TextureFormat,
        clear_color: wgpu::Color,
        blend_state: wgpu::BlendState,
    ) -> Self {
        ////////////////////////////// Shape pipeline /////////////////////////////////
        // TODO: Can I use a single shader here? Should I?
        let shape_shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("Shader"),
            source: wgpu::ShaderSource::Wgsl(include_str!("../shaders/shape_shader.wgsl").into()),
        });

        // TODO: How do the buffers work when I want to update them each frame / don't know ahead of time all the things to draw? How does egui-wgpu do it?
        // Vertex buffer
        let shape_vertex_data_slice = bytemuck::cast_slice(&SHAPE_VERTICES);
        let shape_vertex_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Vertex Buffer"),
            contents: shape_vertex_data_slice,
            usage: wgpu::BufferUsages::VERTEX,
        });

        // Index buffer
        let shape_index_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Index Buffer"),
            contents: bytemuck::cast_slice(SHAPE_INDICES),
            usage: wgpu::BufferUsages::INDEX,
        });
        let shape_num_indices = SHAPE_INDICES.len() as u32;

        // Uniform buffer here.

        // Bind group

        // Shape Pipeline
        let shape_pipeline_layout =
            device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                label: Some("Shape Pipeline Layout"),
                bind_group_layouts: &[],
                push_constant_ranges: &[],
            });

        let shape_pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("Shape Pipeline"),
            layout: Some(&shape_pipeline_layout),
            vertex: wgpu::VertexState {
                module: &shape_shader,
                entry_point: "vs_main",
                // buffers: &[ShapeVertex::desc()],
                buffers: &[Vertex::desc()],
            },
            fragment: Some(wgpu::FragmentState {
                module: &shape_shader,
                entry_point: "fs_main",
                targets: &[Some(wgpu::ColorTargetState {
                    format: surface_format,
                    // blend: Some(wgpu::BlendState {
                    //     color: wgpu::BlendComponent::REPLACE,
                    //     alpha: wgpu::BlendComponent::REPLACE,
                    // }),
                    blend: Some(blend_state),
                    write_mask: wgpu::ColorWrites::ALL,
                })],
            }),
            primitive: wgpu::PrimitiveState {
                topology: wgpu::PrimitiveTopology::TriangleList,
                strip_index_format: None,
                front_face: wgpu::FrontFace::Ccw,
                cull_mode: Some(wgpu::Face::Back),
                // Setting this to anything other than Fill requires Features::POLYGON_MODE_LINE
                // or Features::POLYGON_MODE_POINT
                polygon_mode: wgpu::PolygonMode::Fill,
                // Requires Features::DEPTH_CLIP_CONTROL
                unclipped_depth: false,
                // Requires Features::CONSERVATIVE_RASTERIZATION
                conservative: false,
            },
            depth_stencil: None,
            multisample: wgpu::MultisampleState {
                count: 1,
                mask: !0,
                alpha_to_coverage_enabled: false,
            },
            // If the pipeline will be used with a multiview render pass, this
            // indicates how many array layers the attachments will have.
            multiview: None,
        });

        ////////////////////////////// Sprite pipeline /////////////////////////////////
        // TODO: Can I use a single shader here? Should I?
        let sprite_shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("Sprite Shader"),
            source: wgpu::ShaderSource::Wgsl(include_str!("../shaders/sprite_shader.wgsl").into()),
        });

        // TODO: How do the buffers work when I want to update them each frame / don't know ahead of time all the things to draw? How does egui-wgpu do it?
        // Vertex buffer
        let sprite_vertex_data_slice = bytemuck::cast_slice(&SPRITE_VERTICES);
        let sprite_vertex_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Sprite Vertex Buffer"),
            contents: sprite_vertex_data_slice,
            usage: wgpu::BufferUsages::VERTEX,
        });

        // Index buffer
        let sprite_index_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Sprite Index Buffer"),
            contents: bytemuck::cast_slice(SPRITE_INDICES),
            usage: wgpu::BufferUsages::INDEX,
        });
        let sprite_num_indices = SPRITE_INDICES.len() as u32;

        // Uniform buffer here.

        // Bind group
        let sprite_bind_group_layout =
            device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
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
                        // This should match the filterable field of the
                        // corresponding Texture entry above.
                        ty: wgpu::BindingType::Sampler(wgpu::SamplerBindingType::Filtering),
                        count: None,
                    },
                ],
                label: Some("texture_bind_group_layout"),
            });

        // // This bind group can be swapped out on the fly with other compatible bind groups.
        // let sprite_bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
        //     layout: &sprite_bind_group_layout,
        //     entries: &[
        //         wgpu::BindGroupEntry {
        //             binding: 0,
        //             resource: wgpu::BindingResource::TextureView(&sprite_texture.view),
        //         },
        //         wgpu::BindGroupEntry {
        //             binding: 1,
        //             resource: wgpu::BindingResource::Sampler(&sprite_texture.sampler),
        //         },
        //     ],
        //     label: Some("Sprite Bind Group"),
        // });

        // Sprite Pipeline
        let sprite_pipeline_layout =
            device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                label: Some("Sprite Pipeline Layout"),
                bind_group_layouts: &[&sprite_bind_group_layout],
                push_constant_ranges: &[],
            });

        let sprite_pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("Sprite Pipeline"),
            layout: Some(&sprite_pipeline_layout),
            vertex: wgpu::VertexState {
                module: &sprite_shader,
                entry_point: "vs_main",
                // buffers: &[SpriteVertex::desc()],
                buffers: &[Vertex::desc()],
            },
            fragment: Some(wgpu::FragmentState {
                module: &sprite_shader,
                entry_point: "fs_main",
                targets: &[Some(wgpu::ColorTargetState {
                    format: surface_format,
                    // blend: Some(wgpu::BlendState {
                    //     color: wgpu::BlendComponent::REPLACE,
                    //     alpha: wgpu::BlendComponent::REPLACE,
                    // }),
                    blend: Some(blend_state),
                    write_mask: wgpu::ColorWrites::ALL,
                })],
            }),
            primitive: wgpu::PrimitiveState {
                topology: wgpu::PrimitiveTopology::TriangleList,
                strip_index_format: None,
                front_face: wgpu::FrontFace::Ccw,
                cull_mode: Some(wgpu::Face::Back),
                // Setting this to anything other than Fill requires Features::POLYGON_MODE_LINE
                // or Features::POLYGON_MODE_POINT
                polygon_mode: wgpu::PolygonMode::Fill,
                // Requires Features::DEPTH_CLIP_CONTROL
                unclipped_depth: false,
                // Requires Features::CONSERVATIVE_RASTERIZATION
                conservative: false,
            },
            depth_stencil: None,
            multisample: wgpu::MultisampleState {
                count: 1,
                mask: !0,
                alpha_to_coverage_enabled: false,
            },
            // If the pipeline will be used with a multiview render pass, this
            // indicates how many array layers the attachments will have.
            multiview: None,
        });

        Self {
            shape_vertex_buffer,
            shape_index_buffer,
            // uniform_buffer,
            shape_pipeline,
            clear_color,
            shape_num_indices,
            sprite_vertex_buffer,
            sprite_index_buffer,
            // uniform_buffer,
            sprite_pipeline,
            sprite_num_indices,
            sprite_bind_group_layout,
        }
    }

    // pub fn render(&self, encoder: &mut wgpu::CommandEncoder, render_target: &wgpu::TextureView) {
    //     let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
    //         label: Some("Render Pass"),
    //         color_attachments: &[Some(wgpu::RenderPassColorAttachment {
    //             view: &render_target,
    //             resolve_target: None,
    //             ops: wgpu::Operations {
    //                 load: wgpu::LoadOp::Clear(self.clear_color),
    //                 store: true,
    //             },
    //         })],
    //         depth_stencil_attachment: None,
    //     });

    //     render_pass.set_pipeline(&self.render_pipeline);
    //     render_pass.set_vertex_buffer(0, self.vertex_buffer.slice(..));
    //     render_pass.set_index_buffer(self.index_buffer.slice(..), wgpu::IndexFormat::Uint16);
    //     render_pass.draw_indexed(0..self.num_indices, 0, 0..1);
    // }

    pub fn begin<'pass>(
        &self,
        encoder: &'pass mut wgpu::CommandEncoder,
        render_target: &'pass wgpu::TextureView,
    ) -> RenderPass<'pass> {
        encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
            label: Some("Render Pass"),
            color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                view: &render_target,
                resolve_target: None,
                ops: wgpu::Operations {
                    load: wgpu::LoadOp::Clear(self.clear_color),
                    store: true,
                },
            })],
            depth_stencil_attachment: None,
        })
    }

    pub fn render<'pass>(
        &'pass self,
        render_pass: &mut RenderPass<'pass>,
        shape_bind_group: &'pass wgpu::BindGroup,
        sprite_bind_group: &'pass wgpu::BindGroup,
    ) {
        // We need to loop over all the things we want to render and do these steps for each of them.
        // render_pass.set_pipeline(&self.shape_pipeline);

        // Draw a shape
        render_pass.set_pipeline(&self.sprite_pipeline);
        render_pass.set_bind_group(0, shape_bind_group, &[]);
        render_pass.set_vertex_buffer(0, self.shape_vertex_buffer.slice(..));
        render_pass.set_index_buffer(self.shape_index_buffer.slice(..), wgpu::IndexFormat::Uint16);
        render_pass.draw_indexed(0..self.shape_num_indices, 0, 0..1);

        // Draw a sprite
        render_pass.set_pipeline(&self.sprite_pipeline);
        render_pass.set_bind_group(0, sprite_bind_group, &[]);
        render_pass.set_vertex_buffer(0, self.sprite_vertex_buffer.slice(..));
        render_pass.set_index_buffer(
            self.sprite_index_buffer.slice(..),
            wgpu::IndexFormat::Uint16,
        );
        render_pass.draw_indexed(0..self.sprite_num_indices, 0, 0..1);

        // Draw an outline (not a wireframe)
        // render_pass.set_pipeline(&self.shape_pipeline);
        // render_pass.set_vertex_buffer(0, self.shape_vertex_buffer.slice(..));
        // render_pass.set_index_buffer(self.shape_index_buffer.slice(..), wgpu::IndexFormat::Uint16);
        // render_pass.draw_indexed(0..self.shape_num_indices, 0, 0..1);
        // Here we also need to set the uniform bind group and maybe scissor rect for the rpass?
    }

    pub fn create_sprite_bind_group(
        &self,
        texture: &Texture,
        device: &wgpu::Device,
    ) -> wgpu::BindGroup {
        // This bind group can be swapped out on the fly with other compatible bind groups.
        device.create_bind_group(&wgpu::BindGroupDescriptor {
            layout: &self.sprite_bind_group_layout,
            entries: &[
                wgpu::BindGroupEntry {
                    binding: 0,
                    resource: wgpu::BindingResource::TextureView(&texture.view),
                },
                wgpu::BindGroupEntry {
                    binding: 1,
                    resource: wgpu::BindingResource::Sampler(&texture.sampler),
                },
            ],
            label: Some("Sprite Bind Group"),
        })
    }
}

pub struct Bananas {
    pub device: wgpu::Device,
    pub queue: wgpu::Queue,
    pub surface: wgpu::Surface,
    pub config: wgpu::SurfaceConfiguration,
    pub size: winit::dpi::PhysicalSize<u32>,
}

impl Bananas {
    pub async fn new(window: &Window) -> Self {
        let size = window.inner_size();

        // BackendBit::PRIMARY => Vulkan + Metal + DX12 + Browser WebGPU
        let instance = wgpu::Instance::new(wgpu::Backends::all());
        let surface = unsafe { instance.create_surface(window) };
        let adapter = instance
            .request_adapter(&wgpu::RequestAdapterOptions {
                power_preference: wgpu::PowerPreference::default(),
                compatible_surface: Some(&surface),
                force_fallback_adapter: false,
            })
            .await
            .unwrap();

        let (device, queue) = adapter
            .request_device(
                &wgpu::DeviceDescriptor {
                    label: None,
                    features: wgpu::Features::empty(),
                    limits: wgpu::Limits::default(),
                },
                None, // Trace path
            )
            .await
            .unwrap();

        let config = wgpu::SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format: surface.get_supported_formats(&adapter)[0],
            width: size.width,
            height: size.height,
            present_mode: wgpu::PresentMode::Fifo,
        };
        surface.configure(&device, &config);

        Self {
            device,
            queue,
            surface,
            config,
            size,
        }
    }

    pub fn resize(&mut self, new_size: winit::dpi::PhysicalSize<u32>) {
        if new_size.width > 0 && new_size.height > 0 {
            self.size = new_size;
            self.config.width = new_size.width;
            self.config.height = new_size.height;
            self.surface.configure(&self.device, &self.config);
        }
    }
}
