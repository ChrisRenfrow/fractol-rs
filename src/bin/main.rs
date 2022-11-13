use num::{complex::ComplexFloat, Complex, Num};
use pixels::{Error, Pixels, SurfaceTexture};
use winit::{
    dpi::{LogicalPosition, LogicalSize, PhysicalPosition, PhysicalSize, Pixel, Size},
    event::{DeviceEvent, Event, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::{Window, WindowBuilder},
};

use fractol_rs::{color::Color, utils::linmap};

const WIDTH: u32 = 600;
const HEIGHT: u32 = 400;

fn hello_pixels(pixels: &mut Pixels, window: &Window) {
    let mut frame = pixels.get_frame();
    let fg = [0xff, 0xff, 0xff, 0xff];
    let bg = [0, 0, 0, 0];

    for (i, pixel) in frame.chunks_exact_mut(4).enumerate() {
        pixel.copy_from_slice({
            if i % 100 <= 30 {
                &fg
            } else {
                &bg
            }
        });
    }
}

fn mandelbrot_iterations(cr: f64, ci: f64, max_iter: usize) -> usize {
    let mut z = Complex::new(0.0, 0.0);
    let mut c = Complex::new(cr, ci);

    let mut i: usize = 0;

    for _ in 0..max_iter {
        i += 1;
        z = z * z + c;
        if z.abs() > 4.0 {
            break;
        }
    }

    i
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

    let height = window.inner_size().height;
    let width = window.inner_size().width;

    let x_lower = -2.5;
    let x_upper = 1.05555555;
    let y_lower = -1.0;
    let y_upper = 1.0;
    let mut frame = pixels.get_frame();

    for (i, p) in (0..width * height).zip(frame.chunks_exact_mut(4)) {
        let x = i % width;
        let y = i / width;
        let x1 = linmap(x as f64, (0.0, width as f64), (x_lower, x_upper));
        let y1 = linmap(y as f64, (0.0, height as f64), (y_lower, y_upper));
        let it = mandelbrot_iterations(x1, y1, 255) as u8;
        let color = &[it, it, it, 255];

        p.copy_from_slice(color);
    }

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
            } => {}
            Event::WindowEvent {
                event:
                    WindowEvent::CursorMoved {
                        device_id,
                        position,
                        modifiers,
                    },
                ..
            } => {
                // cursor_pos = position;
            }
            Event::WindowEvent {
                event: WindowEvent::Resized(size),
                ..
            } => {
                // println!("Resize event: {:?}", size);
                // pixels.resize_buffer(size.width, size.height);
                // pixels.resize_surface(size.width, size.height);
            }
            Event::MainEventsCleared => {
                // hello_pixels(&mut pixels, &window);
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
    })
}
