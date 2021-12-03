/// Contains the data required to use OpenGL
mod game;

struct GLContext {
    event_loop: glium::glutin::event_loop::EventLoop<()>,
    display: glium::Display,
}

impl GLContext {
    /// Set up OpenGL in a sane testing configuration
    fn new() -> Self {
        let event_loop = glium::glutin::event_loop::EventLoop::new();
        let wb = glium::glutin::window::WindowBuilder::new()
            .with_inner_size(glium::glutin::dpi::LogicalSize::new(500, 500))
            .with_title("Hello world!");
        let cb = glium::glutin::ContextBuilder::new();
        // Used for building buffers, textures, and drawing
        let display = glium::Display::new(wb, cb, &event_loop).expect("Could not build display");
        GLContext {
            event_loop,
            display,
        }
    }
}

fn main() {
    let GLContext {
        event_loop,
        display,
    } = GLContext::new();
    let mut game_context = game::GameContext::new();
    event_loop.run(move |event, _event_loop_window_target, control_flow| {
        use glium::glutin;

        // Rendering
        let mut target = display.draw();
        game_context.render(&mut target);
        target.finish().expect("finish no work?");

        // By default, just wait until the next frame to render
        let next_frame_time =
            std::time::Instant::now() + std::time::Duration::from_nanos(16_666_667);
        *control_flow = glutin::event_loop::ControlFlow::WaitUntil(next_frame_time);

        // Event processing
        match event {
            glutin::event::Event::WindowEvent {
                event: window_event,
                ..
            } => match window_event {
                glutin::event::WindowEvent::CloseRequested => {
                    *control_flow = glutin::event_loop::ControlFlow::Exit;
                    return;
                }
                glutin::event::WindowEvent::KeyboardInput { input, .. } => {
                    if let Some(virtual_key) = input.virtual_keycode {
                        game_context.press(virtual_key);
                    }
                }
                other_window_event => {
                    println!("Other window event: {:?}", other_window_event);
                }
            },
            other_glutin_event => {
                println!("Other glutin event: {:?}", other_glutin_event);
            }
        }
    });
}
