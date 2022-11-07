use std::iter;

use renderer::{Bananas, Camera, Renderer, ViewProjectionUniform};
use winit::{
    dpi::LogicalSize,
    event::*,
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
};

mod renderer;
mod texture;

fn main() {
    pollster::block_on(run());
}

const ASPECT_RATIO: f32 = 16_f32 / 9_f32;
pub const DEFAULT_WINDOW_WIDTH: u32 = 1024;
pub const DEFAULT_WINDOW_HEIGHT: u32 = (DEFAULT_WINDOW_WIDTH as f32 / ASPECT_RATIO) as u32;

pub async fn run() {
    env_logger::init();

    let event_loop = EventLoop::new();

    let monitor = event_loop
        .available_monitors()
        .next()
        .expect("no monitors found");

    let size = LogicalSize::new(DEFAULT_WINDOW_WIDTH as f64, DEFAULT_WINDOW_HEIGHT as f64);
    let window = WindowBuilder::new()
        .with_title("Papercut")
        .with_inner_size(size)
        .with_min_inner_size(size)
        .with_position(monitor.position())
        .with_visible(false)
        .build(&event_loop)
        .expect("TODO"); //?

    let clear_color = wgpu::Color {
        r: 0.1,
        g: 0.2,
        b: 0.3,
        a: 1.0,
    };
    let blend_state = wgpu::BlendState {
        color: wgpu::BlendComponent::REPLACE,
        alpha: wgpu::BlendComponent::REPLACE,
    };
    let mut bananas = pollster::block_on(Bananas::new(&window)); //.expect("TODO"); //?;
    let mut renderer = Renderer::new(
        &bananas.device,
        bananas.config.format,
        clear_color,
        blend_state,
    );

    ////// Start game state stuff
    let shape_bytes: &[u8] = &[255, 255, 255, 255];
    let shape_texture = texture::Texture::from_bytes(
        &bananas.device,
        &bananas.queue,
        1,
        1,
        shape_bytes,
        Some("shape texture"),
    )
    .expect("TODO");
    let shape_bind_group = renderer.create_sprite_bind_group(&shape_texture, &bananas.device); // TODO: <--- This is all renderer stuff

    let sprite_bytes = include_bytes!("../tree.png");
    let sprite_texture = texture::Texture::from_image_bytes(
        &bananas.device,
        &bananas.queue,
        sprite_bytes,
        "tree.png",
    )
    .expect("TODO");
    let sprite_bind_group = renderer.create_sprite_bind_group(&sprite_texture, &bananas.device);

    let mut camera = Camera::new(size.width as f32, size.height as f32);
    ////// End game state stuff

    window.set_visible(true);
    event_loop.run(move |event, _, control_flow| {
        match event {
            Event::WindowEvent {
                ref event,
                window_id,
            } if window_id == window.id() => {
                // if !state.input(event) {
                match event {
                    WindowEvent::CloseRequested
                    | WindowEvent::KeyboardInput {
                        input:
                            KeyboardInput {
                                state: ElementState::Pressed,
                                virtual_keycode: Some(VirtualKeyCode::Escape),
                                ..
                            },
                        ..
                    } => *control_flow = ControlFlow::Exit,
                    WindowEvent::Resized(physical_size) => {
                        // TODO: Resize should scale the view up or down, not show more or less of it.
                        bananas.resize(*physical_size);
                        camera.resize(physical_size.width as f32, physical_size.height as f32);
                        renderer.resize(&bananas);
                    }
                    WindowEvent::ScaleFactorChanged { new_inner_size, .. } => {
                        // TODO: Resize should scale the view up or down, not show more or less of it.
                        bananas.resize(**new_inner_size);
                        camera.resize(new_inner_size.width as f32, new_inner_size.height as f32);
                        renderer.resize(&bananas);
                    }
                    _ => {}
                }
                // }
            }
            Event::RedrawRequested(window_id) if window_id == window.id() => {
                // state.update();
                match make_piccys(
                    &bananas,
                    &renderer,
                    &shape_bind_group,
                    &sprite_bind_group,
                    &camera,
                ) {
                    Ok(_) => {}
                    // Reconfigure the surface if it's lost or outdated
                    Err(wgpu::SurfaceError::Lost | wgpu::SurfaceError::Outdated) => {
                        bananas.resize(bananas.size);
                        camera.resize(bananas.size.width as f32, bananas.size.height as f32);
                        renderer.resize(&bananas);
                    }
                    // The system is out of memory, we should probably quit
                    Err(wgpu::SurfaceError::OutOfMemory) => *control_flow = ControlFlow::Exit,
                    // We're ignoring timeouts
                    Err(wgpu::SurfaceError::Timeout) => log::warn!("Surface timeout"),
                }
            }
            Event::MainEventsCleared => {
                // RedrawRequested will only trigger once, unless we manually
                // request it.
                window.request_redraw();
            }
            _ => {}
        }
    });
}

fn make_piccys(
    bananas: &Bananas,
    renderer: &Renderer,
    shape_bind_group: &wgpu::BindGroup,
    sprite_bind_group: &wgpu::BindGroup,
    camera: &Camera,
) -> Result<(), wgpu::SurfaceError> {
    // TODO: Textures / sprites
    // TODO: Camera
    let frame = bananas.surface.get_current_texture()?;
    let render_target = frame
        .texture
        .create_view(&wgpu::TextureViewDescriptor::default());

    let mut encoder = bananas
        .device
        .create_command_encoder(&wgpu::CommandEncoderDescriptor {
            label: Some("Render Encoder"),
        });

    let view_projection_uniform = ViewProjectionUniform {
        view: camera.get_view().to_cols_array_2d(),
        projection: camera.get_projection().to_cols_array_2d(),
    };

    let view_projection_uniform_buffer = wgpu::util::DeviceExt::create_buffer_init(
        &bananas.device,
        &wgpu::util::BufferInitDescriptor {
            label: Some("View Projection Uniform Buffer"),
            contents: bytemuck::cast_slice(&[view_projection_uniform]),
            usage: wgpu::BufferUsages::COPY_SRC,
        },
    );

    encoder.copy_buffer_to_buffer(
        &view_projection_uniform_buffer,
        0,
        &renderer.view_projection_uniform_buffer,
        0,
        std::mem::size_of::<ViewProjectionUniform>() as wgpu::BufferAddress,
    );

    {
        let mut render_pass = renderer.begin(&mut encoder, &render_target);
        // let mut gfx = renderer.begin(&mut encoder, &render_target); ???
        // gfx.draw_shape(shape); ???
        // gfx.draw_sprite(sprite); ???
        // renderer.end(gfx); ???
        renderer.render(&mut render_pass, shape_bind_group, sprite_bind_group);
    }

    bananas.queue.submit(iter::once(encoder.finish()));
    frame.present();

    Ok(())
}

// struct Shape {}
