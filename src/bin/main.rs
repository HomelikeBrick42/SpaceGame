use space::game::Game;
use std::sync::Arc;
use winit::{
    dpi::PhysicalSize,
    event::{Event, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
};

fn main() {
    let event_loop = EventLoop::new().unwrap();
    let window = Arc::new(
        WindowBuilder::new()
            .with_title("Space")
            .with_visible(false)
            .build(&event_loop)
            .unwrap(),
    );

    let mut game = pollster::block_on(Game::new(window.clone()));

    event_loop.set_control_flow(ControlFlow::Poll);
    let start_time = std::time::Instant::now();
    let mut last_time = start_time;
    window.set_visible(true);
    event_loop
        .run(move |event, elwt| match event {
            Event::WindowEvent {
                event, window_id, ..
            } if window.id() == window_id => match event {
                WindowEvent::CloseRequested => {
                    window.set_visible(false);
                    elwt.exit();
                }
                WindowEvent::Resized(PhysicalSize { width, height }) => game.resize(width, height),
                WindowEvent::RedrawRequested => game.render(),
                _ => {}
            },
            Event::AboutToWait => {
                let time = std::time::Instant::now();
                let dt = time.duration_since(last_time);
                last_time = time;

                game.update(time.duration_since(start_time), dt);
                window.request_redraw();
            }
            _ => (),
        })
        .unwrap();
}
