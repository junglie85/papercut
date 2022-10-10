use flexi_logger::Logger;
use pixels::{Pixels, SurfaceTexture};
use winit::{
    dpi::LogicalSize,
    event::Event,
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
};
use winit_input_helper::WinitInputHelper;

const WIDTH: u32 = 1024;
const HEIGHT: u32 = 768;

fn main() -> anyhow::Result<()> {
    let max_level = "info";
    Logger::try_with_str(format!("{}, wgpu_core=warn, wgpu_hal=error", max_level))?.start()?;

    let event_loop = EventLoop::new();
    let window = {
        let size = LogicalSize::new(WIDTH as f64, HEIGHT as f64);
        WindowBuilder::new()
            .with_title("Papercut")
            .with_inner_size(size)
            .build(&event_loop)?
    };

    let mut input = WinitInputHelper::new();

    let mut pixels = {
        let window_size = window.inner_size();
        let surface_texture = SurfaceTexture::new(window_size.width, window_size.height, &window);
        Pixels::new(WIDTH, HEIGHT, surface_texture)?
    };

    let app = App::default();

    let mut destroying = false;
    event_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Poll;

        if let Event::RedrawRequested(window_id) = event {
            if !destroying && window_id == window.id() {
                app.render(pixels.get_frame_mut());
                if let Err(e) = pixels.render() {
                    log::error!("{}", e);
                    *control_flow = ControlFlow::Exit;
                    return;
                }
            }
        }

        let processed_all_events = input.update(&event);
        if processed_all_events {
            if input.quit() && !destroying {
                destroying = true;
                *control_flow = ControlFlow::Exit;
                app.destroy();
            }

            if let Some(_) = input.scale_factor_changed() {
                let size = window.inner_size();
                app.resize(size);
                pixels.resize_surface(size.width, size.height);
            }

            if let Some(size) = input.window_resized() {
                app.resize(size);
                pixels.resize_surface(size.width, size.height);
            }

            if !destroying {
                app.update(); // TODO: Update loop timing
                window.request_redraw();
            }
        }
    });
}

#[derive(Debug, Default)]
struct App {}

impl App {
    fn update(&self) {}

    fn render(&self, data: &mut [u8]) {
        for pixel in data.chunks_exact_mut(4) {
            let rgba = [0x33, 0x33, 0x33, 0xff];

            pixel.copy_from_slice(&rgba);
        }
    }

    fn resize(&self, _size: winit::dpi::PhysicalSize<u32>) {}

    fn destroy(&self) {}
}
