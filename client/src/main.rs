use pixels::{Pixels, SurfaceTexture};
use std::time::Instant;
use winit::{
    event::{Event, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
};
use common::font::draw_text;

const WIDTH: u32  = 800;
const HEIGHT: u32 = 800;

fn main() {
    let event_loop = EventLoop::new();

    let window = WindowBuilder::new()
        .with_title("Maze War")
        .with_inner_size(winit::dpi::LogicalSize::new(WIDTH as f64, HEIGHT as f64))
        .with_resizable(false)
        .build(&event_loop)
        .unwrap();

    let mut pixels = {
        let surface = SurfaceTexture::new(WIDTH, HEIGHT, &window);
        Pixels::new(WIDTH, HEIGHT, surface).unwrap()
    };

    let mut frames = 0u32;
    let mut last   = Instant::now();
    let mut fps    = 0u32;

    event_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Poll;

        match event {
            Event::MainEventsCleared => window.request_redraw(),

            Event::RedrawRequested(_) => {
                frames += 1;
                if last.elapsed().as_secs_f32() >= 1.0 {
                    fps    = frames;
                    frames = 0;
                    last   = Instant::now();
                }

                let frame = pixels.frame_mut();
                frame.fill(0);
                draw_text(frame, WIDTH as usize, &format!("FPS:{}", fps), 4, 4, [255, 255, 255, 255], 1.0);

                pixels.render().unwrap();
            }

            Event::WindowEvent { event, .. } => match event {
                WindowEvent::CloseRequested => *control_flow = ControlFlow::Exit,
                _ => {}
            },

            _ => {}
        }
    });
}