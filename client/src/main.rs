use pixels::{Pixels, SurfaceTexture};
use winit::{
    event::{Event, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
};

fn main() {
    let event_loop = EventLoop::new();

    let window = WindowBuilder::new()
        .with_title("Maze War")
        .with_resizable(false)
        .build(&event_loop)
        .unwrap();

    let size = window.inner_size();
    let mut pixels = {
        let surface = SurfaceTexture::new(size.width, size.height, &window);
        Pixels::new(size.width, size.height, surface).unwrap()
    };

    event_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Poll;

        match event {
            Event::MainEventsCleared => window.request_redraw(),

            Event::RedrawRequested(_) => {
                pixels.frame_mut().fill(0);
                pixels.render().unwrap();
            }

            Event::WindowEvent { event, .. } => match event {
                WindowEvent::CloseRequested => *control_flow = ControlFlow::Exit,
                WindowEvent::Resized(size) => pixels.resize_surface(size.width, size.height).unwrap(),
                _ => {}
            },

            _ => {}
        }
    });
}