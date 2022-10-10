use editor::Editor;
use flexi_logger::Logger;
use log::error;
use pixels::{PixelsBuilder, SurfaceTexture};
use winit::{
    dpi::LogicalSize,
    event::{DeviceEvent, ElementState, Event, KeyboardInput},
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
};
use winit_input_helper::WinitInputHelper;

mod editor;

/* TODO's:
 * - Handle exiting and entering fullscreen
 * - Better monitor detection and setup
 *   - Game mode: use Exclusive fullscreen? Or give user the option?
 *   - Editor mode: use Borderless fullscreen
 *   - Figure out pixels surface sizing, as always want a full window
 * - Remember window location, size, etc, between restarts
 *   - Check if able to open in those positions, otherwise pick a reasonabel default
 *   - Store settings in proper place for each OS
 */

const WIDTH: u32 = 960;
const HEIGHT: u32 = 540;

fn main() -> anyhow::Result<()> {
    let max_level = "info";
    Logger::try_with_str(format!("{}, wgpu_core=warn, wgpu_hal=error", max_level))?.start()?;

    let event_loop = EventLoop::new();

    let monitor = event_loop
        .available_monitors()
        .next()
        .expect("no monitors found");

    let window = {
        let monitor_size = monitor.size();
        let (mut width, mut height) = (WIDTH, HEIGHT);
        loop {
            if width * 2 <= monitor_size.width && height * 2 <= monitor_size.height {
                width *= 2;
                height *= 2;
            } else {
                break;
            }
        }
        let size = LogicalSize::new(width as f64, height as f64);
        WindowBuilder::new()
            .with_title("Papercut")
            .with_inner_size(size)
            .with_min_inner_size(size)
            .with_max_inner_size(size)
            .with_position(monitor.position())
            .with_visible(false)
            .build(&event_loop)?
    };

    let mut input = WinitInputHelper::new();

    let (mut pixels, mut editor) = {
        let size = window.inner_size();
        let scale_factor = window.scale_factor() as f32;
        let surface_texture = SurfaceTexture::new(size.width, size.height, &window);
        let pixels = pollster::block_on(
            PixelsBuilder::new(size.width, size.height, surface_texture).build_async(),
        )?;
        let editor = Editor::new(&event_loop, size.width, size.height, scale_factor, &pixels);

        (pixels, editor)
    };

    let app = App::default();

    window.set_visible(true);
    let mut destroying = false;
    event_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Poll;

        let processed_all_events = input.update(&event);
        if processed_all_events {
            if input.quit() && !destroying {
                destroying = true;
                *control_flow = ControlFlow::Exit;
                app.destroy();
            }

            if let Some(scale_factor) = input.scale_factor_changed() {
                editor.scale_factor(scale_factor)
            }

            if let Some(size) = input.window_resized() {
                app.resize(size);
                pixels.resize_surface(size.width, size.height);
                editor.resize(size.width, size.height);
            }

            if !destroying {
                app.update(); // TODO: Update loop timing
                window.request_redraw();
            }
        }

        match event {
            Event::DeviceEvent {
                event:
                    DeviceEvent::Key(KeyboardInput {
                        scancode: 41, // Grave - doesn't register as KeyboardInput event for some reason.
                        state: ElementState::Pressed,
                        ..
                    }),
                ..
            } => {
                // TODO: Ideally this would be a double press of the Grave key; use own input helper.
                editor.toggle();
            }
            Event::WindowEvent { window_id, event } if window_id == window.id() => {
                editor.handle_event(&event);
            }
            Event::RedrawRequested(window_id) if !destroying && window_id == window.id() => {
                app.draw(pixels.get_frame_mut());
                editor.draw(&window);

                let render_result = pixels.render_with(|encoder, render_target, context| {
                    context.scaling_renderer.render(encoder, render_target);
                    editor.render(encoder, render_target, context);

                    Ok(())
                });

                if let Err(e) = render_result {
                    error!("{}", e);
                    *control_flow = ControlFlow::Exit;
                    return;
                }
            }
            _ => {}
        }
    });
}

#[derive(Debug, Default)]
struct App {}

impl App {
    fn update(&self) {}

    fn draw(&self, data: &mut [u8]) {
        for pixel in data.chunks_exact_mut(4) {
            let rgba = [0x33, 0x33, 0x33, 0xff];

            pixel.copy_from_slice(&rgba);
        }
    }

    fn resize(&self, _size: winit::dpi::PhysicalSize<u32>) {}

    fn destroy(&self) {}
}
