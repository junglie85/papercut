use editor::Editor;
use flexi_logger::Logger;
use log::error;
use pixels::{PixelsBuilder, SurfaceTexture};
use renderer::Renderer;
use winit::{
    dpi::LogicalSize,
    event::{DeviceEvent, ElementState, Event, KeyboardInput},
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
};
use winit_input_helper::WinitInputHelper;

mod editor;
mod renderer;

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

const ASPECT_RATIO: f32 = 16_f32 / 9_f32;
const WIDTH: u32 = 1024;
const HEIGHT: u32 = (WIDTH as f32 / ASPECT_RATIO) as u32;

fn main() -> anyhow::Result<()> {
    let max_level = "info";
    Logger::try_with_str(format!("{}, wgpu_core=warn, wgpu_hal=error", max_level))?.start()?;

    let event_loop = EventLoop::new();

    let monitor = event_loop
        .available_monitors()
        .next()
        .expect("no monitors found");

    let window = {
        let size = LogicalSize::new(WIDTH as f64, HEIGHT as f64);
        WindowBuilder::new()
            .with_title("Papercut")
            .with_inner_size(size)
            .with_min_inner_size(size)
            .with_position(monitor.position())
            .with_visible(false)
            .build(&event_loop)?
    };

    let mut input = WinitInputHelper::new();

    let (mut pixels, mut editor) = {
        let size = window.inner_size();
        let scale_factor = window.scale_factor() as f32;
        let surface_texture = SurfaceTexture::new(WIDTH, HEIGHT, &window);
        let pixels =
            pollster::block_on(PixelsBuilder::new(WIDTH, HEIGHT, surface_texture).build_async())?;
        let editor = Editor::new(&event_loop, size.width, size.height, scale_factor, &pixels);

        (pixels, editor)
    };

    let mut renderer = {
        let size = window.inner_size();
        Renderer::new(&pixels, WIDTH, HEIGHT, size.width, size.height)
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

                let clip_rect = pixels.context().scaling_renderer.clip_rect();
                renderer.resize(&pixels, clip_rect.2, clip_rect.3, size.width, size.height);

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
                let mut buffer = Buffer {
                    width: WIDTH,
                    height: HEIGHT,
                    data: pixels.get_frame_mut(),
                };
                app.draw(&mut buffer);
                editor.draw(&window);

                let render_result = pixels.render_with(|encoder, render_target, context| {
                    let fill_texture = renderer.get_texture_view();
                    context.scaling_renderer.render(encoder, fill_texture);

                    renderer.render(encoder, render_target);
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

    fn draw(&self, buffer: &mut Buffer) {
        let rgba = [0x33, 0x33, 0x33, 0xff];
        Gfx::clear(buffer, rgba);
        Gfx::draw_square(buffer, 50, 500, 10, 10, [0xff, 0x00, 0x00, 0xff]);
    }

    fn resize(&self, _size: winit::dpi::PhysicalSize<u32>) {}

    fn destroy(&self) {}
}

struct Buffer<'a> {
    width: u32,
    height: u32,
    data: &'a mut [u8],
}

struct Gfx {}

impl Gfx {
    pub fn clear(buffer: &mut Buffer, rgba: [u8; 4]) {
        for y_ in 0..buffer.height {
            for x_ in 0..buffer.width {
                let i = (y_ * buffer.width + x_) as usize * 4;
                let pixel = &mut buffer.data[i..i + 4];
                pixel.copy_from_slice(&rgba)
            }
        }
    }

    pub fn draw_square(
        buffer: &mut Buffer,
        x: i32,
        y: i32,
        width: i32,
        height: i32,
        rgba: [u8; 4],
    ) {
        for y_ in y..(y + height) {
            for x_ in x..(x + width) {
                let i = (y_ * buffer.width as i32 + x_) as usize * 4;
                let pixel = &mut buffer.data[i..i + 4];
                pixel.copy_from_slice(&rgba)
            }
        }
    }
}
