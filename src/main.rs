use pixels::{Error, Pixels, SurfaceTexture};
use winit::{
    dpi::{LogicalSize, PhysicalPosition, PhysicalSize, Pixel},
    event::{DeviceEvent, Event, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::{Window, WindowBuilder},
};

use fractol_rs::color::Color;

const WIDTH: u32 = 400;
const HEIGHT: u32 = 300;

const BASE_COLOR: Color = Color {
    r: 0x00,
    g: 0x00,
    b: 0x00,
    a: 0xff,
};

fn color_for_coords(window: &Window, x: f64, y: f64) -> Color {
    let size = window.inner_size();
    let x_ratio = x / size.width as f64;
    let y_ratio = y / size.height as f64;
    BASE_COLOR.add(&Color::new(
        (255.0 * x_ratio) as u8,
        0x00,
        (255.0 * y_ratio) as u8,
        0x00,
    ))
}

fn main() -> Result<(), Error> {
    let event_loop = EventLoop::new();

    let window = {
        let size = LogicalSize::new(WIDTH, HEIGHT);
        WindowBuilder::new()
            .with_title("fractol-rs")
            .with_inner_size(size)
            .build(&event_loop)
            .unwrap()
    };

    let mut pixels = {
        let window_size = window.inner_size();
        let surface_texture = SurfaceTexture::new(window_size.width, window_size.height, &window);
        Pixels::new(window_size.width, window_size.height, surface_texture)?
    };

    let mut cursor_pos: PhysicalPosition<f64> = PhysicalPosition { x: 0.0, y: 0.0 };

    event_loop.run(move |event, _, control_flow| {
        control_flow.set_poll();

        match event {
            Event::WindowEvent {
                event: WindowEvent::CloseRequested,
                ..
            } => {
                println!("Window close requested, exiting...");
                control_flow.set_exit();
            }
            Event::WindowEvent {
                event:
                    WindowEvent::AxisMotion {
                        device_id,
                        axis,
                        value,
                    },
                ..
            } => {
                // println!(
                //     "Axis Motion Event\ndevice id: {:?}, axis: {}, value: {}",
                //     device_id, axis, value
                // );
            }
            Event::WindowEvent {
                event:
                    WindowEvent::CursorMoved {
                        device_id,
                        position,
                        modifiers,
                    },
                ..
            } => {
                // println!(
                //     "Cursor Moved Event\ndevice id: {:?}, position: {:?}, modifiers: {:?}",
                //     device_id, position, modifiers
                // );
                cursor_pos = position;
            }
            Event::WindowEvent {
                event: WindowEvent::Resized(size),
                ..
            } => {
                println!("Resize event: {:?}", size);
                pixels.resize_buffer(size.width, size.height);
                pixels.resize_surface(size.width, size.height);
            }
            Event::MainEventsCleared => {
                // Clear the pixel buffer
                let frame = pixels.get_frame();
                let color = color_for_coords(&window, cursor_pos.x, cursor_pos.y);
                // println!("{:?}", color);
                for (i, pixel) in frame.chunks_exact_mut(4).enumerate() {
                    pixel[0] = color.r; // R
                    pixel[1] = color.g; // G
                    pixel[2] = color.b; // B
                    pixel[3] = color.a; // A
                }

                if pixels
                    .render()
                    .map_err(|e| eprintln!("pixels.render() failed: {}", e))
                    .is_err()
                {
                    *control_flow = ControlFlow::Exit;
                    return;
                }
            }
            _ => (),
        }
    });
}
