use std::iter;

use renderer::{Bananas, Renderer};
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
const WIDTH: u32 = 1024;
const HEIGHT: u32 = (WIDTH as f32 / ASPECT_RATIO) as u32;

pub async fn run() {
    env_logger::init();

    let event_loop = EventLoop::new();

    let monitor = event_loop
        .available_monitors()
        .next()
        .expect("no monitors found");

    let size = LogicalSize::new(WIDTH as f64, HEIGHT as f64);
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
    let renderer = Renderer::new(
        &bananas.device,
        bananas.config.format,
        clear_color,
        blend_state,
    );

    ////// Start game state stuff
    let sprite_bytes = include_bytes!("../tree.png");
    let sprite_texture =
        texture::Texture::from_bytes(&bananas.device, &bananas.queue, sprite_bytes, "tree.png")
            .expect("TODO");
    let sprite_bind_group = renderer.create_sprite_bind_group(&sprite_texture, &bananas.device);
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
                        bananas.resize(*physical_size);
                    }
                    WindowEvent::ScaleFactorChanged { new_inner_size, .. } => {
                        // new_inner_size is &mut so w have to dereference it twice
                        bananas.resize(**new_inner_size);
                    }
                    _ => {}
                }
                // }
            }
            Event::RedrawRequested(window_id) if window_id == window.id() => {
                // state.update();
                match make_piccys(&bananas, &renderer, &sprite_bind_group) {
                    Ok(_) => {}
                    // Reconfigure the surface if it's lost or outdated
                    Err(wgpu::SurfaceError::Lost | wgpu::SurfaceError::Outdated) => {
                        bananas.resize(bananas.size)
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
    sprite_bind_group: &wgpu::BindGroup,
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

    {
        let mut render_pass = renderer.begin(&mut encoder, &render_target);
        // let mut gfx = renderer.begin(&mut encoder, &render_target); ???
        // gfx.draw_shape(shape); ???
        // gfx.draw_sprite(sprite); ???
        // renderer.end(gfx); ???
        renderer.render(&mut render_pass, sprite_bind_group);
    }

    bananas.queue.submit(iter::once(encoder.finish()));
    frame.present();

    Ok(())
}

// struct Shape {}
