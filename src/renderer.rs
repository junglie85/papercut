use std::ops::Range;

use glam::Mat4;
use lyon::{
    geom::{point, Box2D},
    lyon_tessellation::{
        BuffersBuilder, FillOptions, FillTessellator, FillVertexConstructor, StrokeOptions,
        StrokeTessellator, StrokeVertexConstructor, VertexBuffers,
    },
    path::{Path, Winding},
};
use wgpu::{util::DeviceExt, RenderPass};
use winit::window::Window;

use crate::{texture::Texture, DEFAULT_WINDOW_HEIGHT, DEFAULT_WINDOW_WIDTH};

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
        position: [-8.68241, 49.240386, 0.0],
        tex_coords: [0.0, 0.0],
        color: [0.5, 0.0, 0.5],
    }, // A
    Vertex {
        position: [-49.513406, 6.958647, 0.0],
        tex_coords: [0.0, 0.0],
        color: [0.5, 0.0, 0.5],
    }, // B
    Vertex {
        position: [-21.918549, -44.939706, 0.0],
        tex_coords: [0.0, 0.0],
        color: [0.5, 0.0, 0.5],
    }, // C
    Vertex {
        position: [35.966998, -34.73291, 0.0],
        tex_coords: [0.0, 0.0],
        color: [0.5, 0.0, 0.5],
    }, // D
    Vertex {
        position: [44.147372, 23.47359, 0.0],
        tex_coords: [0.0, 0.0],
        color: [0.5, 0.0, 0.5],
    }, // E
];

const SHAPE_INDICES: &[u16] = &[0, 1, 4, 1, 2, 4, 2, 3, 4, 0];

const SPRITE_VERTICES: &[Vertex] = &[
    Vertex {
        position: [-25.0, 25.0, 0.0],
        tex_coords: [0.0, 0.0],
        color: [1.0, 1.0, 1.0],
    }, // A
    Vertex {
        position: [-25.0, -75.0, 0.0],
        tex_coords: [0.0, 1.0],
        color: [1.0, 1.0, 1.0],
    }, // B
    Vertex {
        position: [75.0, 25.0, 0.0],
        tex_coords: [1.0, 0.0],
        color: [1.0, 1.0, 1.0],
    }, // C
    Vertex {
        position: [75.0, -75.0, 0.0],
        tex_coords: [1.0, 1.0],
        color: [1.0, 1.0, 1.0],
    }, // D
];

const SPRITE_INDICES: &[u16] = &[0, 1, 2, 2, 1, 3];

#[repr(C)]
#[derive(Copy, Clone, Debug)]
struct GpuVertex {
    position: [f32; 3],
    color: [f32; 4],
}

unsafe impl bytemuck::Pod for GpuVertex {}
unsafe impl bytemuck::Zeroable for GpuVertex {}

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

    pub view_projection_uniform_buffer: wgpu::Buffer,
    pub uniforms_bind_group: wgpu::BindGroup,
    depth_texture_view: Option<wgpu::TextureView>,

    geometry_render_pipeline: wgpu::RenderPipeline,
    geometry_vbo: wgpu::Buffer,
    geometry_ibo: wgpu::Buffer,
    geometry_fill_range: Range<u32>,
    geometry_stroke_range: Range<u32>,
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
        let depth_stencil_state = Some(wgpu::DepthStencilState {
            format: wgpu::TextureFormat::Depth32Float,
            depth_write_enabled: true,
            depth_compare: wgpu::CompareFunction::Greater,
            stencil: wgpu::StencilState {
                front: wgpu::StencilFaceState::IGNORE,
                back: wgpu::StencilFaceState::IGNORE,
                read_mask: 0,
                write_mask: 0,
            },
            bias: wgpu::DepthBiasState::default(),
        });

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
            depth_stencil: depth_stencil_state.clone(),
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

        // Uniform buffer
        let view_projection_uniform_buffer = device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("View Projection Uniform Buffer"),
            size: std::mem::size_of::<ViewProjectionUniform>() as wgpu::BufferAddress,
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });

        // Bind groups
        let uniforms_bind_group_layout =
            device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
                label: Some("Uniforms Bind Group Layout"),
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

        let uniforms_bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("Uniforms Bind Group"),
            layout: &uniforms_bind_group_layout,
            entries: &[wgpu::BindGroupEntry {
                binding: 0,
                resource: view_projection_uniform_buffer.as_entire_binding(),
            }],
        });

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
                bind_group_layouts: &[&uniforms_bind_group_layout, &sprite_bind_group_layout],
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
            depth_stencil: depth_stencil_state.clone(),
            multisample: wgpu::MultisampleState {
                count: 1,
                mask: !0,
                alpha_to_coverage_enabled: false,
            },
            // If the pipeline will be used with a multiview render pass, this
            // indicates how many array layers the attachments will have.
            multiview: None,
        });

        /////////////////////////////// Geometry pipeline ///////////////////////////////////
        let tolerance = 0.02;

        let mut geometry: VertexBuffers<GpuVertex, u16> = VertexBuffers::new();

        let mut fill_tess = FillTessellator::new();
        let mut stroke_tess = StrokeTessellator::new();

        let rect = Box2D::new(point(0.0, 0.0), point(500.0, 500.0));
        let mut builder = Path::builder();
        builder.add_rectangle(&rect, Winding::Negative);
        let path = builder.build();

        fill_tess
            .tessellate_path(
                &path,
                &FillOptions::tolerance(tolerance)
                    .with_fill_rule(lyon::tessellation::FillRule::NonZero),
                &mut BuffersBuilder::new(&mut geometry, WithId),
            )
            .unwrap();

        let geometry_fill_range = 0..(geometry.indices.len() as u32);

        stroke_tess
            .tessellate_path(
                &path,
                &StrokeOptions::tolerance(tolerance),
                &mut BuffersBuilder::new(&mut geometry, WithId),
            )
            .unwrap();

        let geometry_stroke_range = geometry_fill_range.end..(geometry.indices.len() as u32);

        dbg!(&geometry);

        let geometry_vbo = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: None,
            contents: bytemuck::cast_slice(&geometry.vertices),
            usage: wgpu::BufferUsages::VERTEX,
        });

        let geometry_ibo = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: None,
            contents: bytemuck::cast_slice(&geometry.indices),
            usage: wgpu::BufferUsages::INDEX,
        });

        let geometry_vs_module = &device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("Geometry vs"),
            source: wgpu::ShaderSource::Wgsl(include_str!("./../shaders/geometry.wgsl").into()),
        });

        let geometry_fs_module = &device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("Geometry fs"),
            source: wgpu::ShaderSource::Wgsl(include_str!("./../shaders/geometry.wgsl").into()),
        });

        let geometry_pipeline_layout =
            device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                bind_group_layouts: &[&uniforms_bind_group_layout],
                push_constant_ranges: &[],
                label: None,
            });

        let geometry_render_pipeline =
            device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
                label: Some("Geometry pipeline"),
                layout: Some(&geometry_pipeline_layout),
                vertex: wgpu::VertexState {
                    module: &geometry_vs_module,
                    entry_point: "vs_main",
                    buffers: &[wgpu::VertexBufferLayout {
                        array_stride: std::mem::size_of::<GpuVertex>() as u64,
                        step_mode: wgpu::VertexStepMode::Vertex,
                        attributes: &[
                            wgpu::VertexAttribute {
                                offset: 0,
                                format: wgpu::VertexFormat::Float32x3,
                                shader_location: 0,
                            },
                            wgpu::VertexAttribute {
                                offset: 12,
                                format: wgpu::VertexFormat::Float32x4,
                                shader_location: 1,
                            },
                        ],
                    }],
                },
                fragment: Some(wgpu::FragmentState {
                    module: &geometry_fs_module,
                    entry_point: "fs_main",
                    targets: &[Some(wgpu::ColorTargetState {
                        format: surface_format,
                        blend: Some(blend_state),
                        write_mask: wgpu::ColorWrites::ALL,
                    })],
                }),
                primitive: wgpu::PrimitiveState {
                    topology: wgpu::PrimitiveTopology::TriangleList,
                    polygon_mode: wgpu::PolygonMode::Fill,
                    front_face: wgpu::FrontFace::Ccw,
                    strip_index_format: None,
                    cull_mode: Some(wgpu::Face::Back),
                    conservative: false,
                    unclipped_depth: false,
                },
                depth_stencil: depth_stencil_state.clone(),
                multisample: wgpu::MultisampleState {
                    count: 1,
                    mask: !0,
                    alpha_to_coverage_enabled: false,
                },
                multiview: None,
            });

        let depth_texture_view = None;

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

            uniforms_bind_group,
            view_projection_uniform_buffer,
            depth_texture_view,

            geometry_render_pipeline,
            geometry_vbo,
            geometry_ibo,
            geometry_fill_range,
            geometry_stroke_range,
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

    pub fn resize(&mut self, bananas: &Bananas) {
        let depth_texture = bananas.device.create_texture(&wgpu::TextureDescriptor {
            label: Some("Depth texture"),
            size: wgpu::Extent3d {
                width: bananas.config.width,
                height: bananas.config.height,
                depth_or_array_layers: 1,
            },
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format: wgpu::TextureFormat::Depth32Float,
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
        });

        self.depth_texture_view =
            Some(depth_texture.create_view(&wgpu::TextureViewDescriptor::default()));
    }

    pub fn begin<'pass>(
        &'pass self,
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
            // depth_stencil_attachment: None,
            depth_stencil_attachment: Some(wgpu::RenderPassDepthStencilAttachment {
                view: self.depth_texture_view.as_ref().expect("TODO"),
                depth_ops: Some(wgpu::Operations {
                    load: wgpu::LoadOp::Clear(0.0),
                    store: true,
                }),
                stencil_ops: Some(wgpu::Operations {
                    load: wgpu::LoadOp::Clear(0),
                    store: true,
                }),
            }),
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

        // // Draw a shape
        // render_pass.set_pipeline(&self.sprite_pipeline);
        // render_pass.set_bind_group(0, &self.uniforms_bind_group, &[]);
        // render_pass.set_bind_group(1, shape_bind_group, &[]);
        // render_pass.set_vertex_buffer(0, self.shape_vertex_buffer.slice(..));
        // render_pass.set_index_buffer(self.shape_index_buffer.slice(..), wgpu::IndexFormat::Uint16);
        // render_pass.draw_indexed(0..self.shape_num_indices, 0, 0..1);

        // // Draw a sprite
        // render_pass.set_pipeline(&self.sprite_pipeline);
        // render_pass.set_bind_group(0, &self.uniforms_bind_group, &[]);
        // render_pass.set_bind_group(1, sprite_bind_group, &[]);
        // render_pass.set_vertex_buffer(0, self.sprite_vertex_buffer.slice(..));
        // render_pass.set_index_buffer(
        //     self.sprite_index_buffer.slice(..),
        //     wgpu::IndexFormat::Uint16,
        // );
        // render_pass.draw_indexed(0..self.sprite_num_indices, 0, 0..1);

        // Draw the tessellated geometry
        render_pass.set_pipeline(&self.geometry_render_pipeline);
        render_pass.set_bind_group(0, &self.uniforms_bind_group, &[]);
        render_pass.set_index_buffer(self.geometry_ibo.slice(..), wgpu::IndexFormat::Uint16);
        render_pass.set_vertex_buffer(0, self.geometry_vbo.slice(..));
        render_pass.draw_indexed(self.geometry_fill_range.clone(), 0, 0..1);
        render_pass.draw_indexed(self.geometry_stroke_range.clone(), 0, 0..1);

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

pub struct WithId;

// var transformed_pos = world_pos * vec3<f32>(globals.zoom / (0.5 * globals.resolution.x), globals.zoom / (0.5 * globals.resolution.y), 1.0);
// TODO: Pass in color and ZIndex
impl FillVertexConstructor<GpuVertex> for WithId {
    fn new_vertex(&mut self, vertex: lyon::tessellation::FillVertex) -> GpuVertex {
        let p = vertex.position().to_array();
        let z_index = 0.0; // 1.0;
        GpuVertex {
            position: [p[0], p[1], z_index],
            color: [1.0, 1.0, 1.0, 1.0],
        }
    }
}

// TODO: We want the color, ZIndex and the width passed in.
impl StrokeVertexConstructor<GpuVertex> for WithId {
    fn new_vertex(&mut self, vertex: lyon::tessellation::StrokeVertex) -> GpuVertex {
        let stroke_width = 1.0;
        let p = (vertex.position() + vertex.normal() * stroke_width).to_array();
        let z_index = 0.0; // 2.0;
        GpuVertex {
            position: [p[0], p[1], z_index],
            color: [0.0, 0.0, 0.0, 1.0],
        }
    }
}

#[repr(C)]
#[derive(Debug, Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
pub struct ViewProjectionUniform {
    pub(crate) view: [[f32; 4]; 4],
    pub(crate) projection: [[f32; 4]; 4],
}

#[allow(dead_code)]
pub struct Camera {
    width: f32,
    height: f32,
    view: Mat4,
    projection: Mat4,
}

impl Camera {
    pub fn new(width: f32, height: f32) -> Self {
        let projection = glam::Mat4::orthographic_lh(0.0, width, 0.0, height, -1.0, 1.0);

        Self {
            width,
            height,
            view: Mat4::IDENTITY,
            projection,
        }
    }

    pub fn resize(&mut self, width: f32, height: f32) {
        let projection = glam::Mat4::orthographic_lh(0.0, width, 0.0, height, -1.0, 1.0);

        self.width = width;
        self.height = height;
        self.projection = projection;
    }

    pub fn get_view(&self) -> Mat4 {
        // Just use some jankey values for look at for now.
        let view = glam::Mat4::look_at_lh(
            glam::Vec3::new(-200.0, -200.0, -1.0),
            glam::Vec3::new(-200.0, -200.0, 0.0),
            glam::Vec3::Y,
        );

        // let view = glam::Mat4::look_at_lh(
        //     glam::Vec3::new(0.0, 0.0, -1.0),
        //     glam::Vec3::new(0.0, 0.0, 0.0),
        //     glam::Vec3::Y,
        // );

        view
    }

    pub fn get_projection(&self) -> Mat4 {
        self.projection
    }
}
