/// Contains the data required to use OpenGL
mod game;

struct GLContext {
    event_loop: glium::glutin::event_loop::EventLoop<()>,
    display: glium::Display,
    starting_scale_factor: f64,
}

impl GLContext {
    /// Set up OpenGL in a sane testing configuration
    fn new() -> Self {
        use glium::glutin;
        let event_loop = glutin::event_loop::EventLoop::new();
        let wb = glutin::window::WindowBuilder::new()
            .with_inner_size(glutin::dpi::LogicalSize::new(500, 500))
            .with_title("Hello world!");
        let wc = glutin::ContextBuilder::new()
            .build_windowed(wb, &event_loop)
            .expect("should be able to build windowed context");
        let starting_scale_factor = wc.window().scale_factor();
        // Used for building buffers, textures, and drawing
        let display = glium::Display::from_gl_window(wc).expect("Could not build display");
        GLContext {
            event_loop,
            display,
            starting_scale_factor,
        }
    }
}

fn main() {
    let GLContext {
        event_loop,
        mut display,
        starting_scale_factor,
    } = GLContext::new();
    let mut scale_factor = starting_scale_factor;
    let mut game_context = game::GameContext::new();
    event_loop.run(move |event, _event_loop_window_target, control_flow| {
        use glium::glutin;
        use glutin::dpi::LogicalPosition;

        // Rendering
        game_context.render(&mut display);

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
                glutin::event::WindowEvent::Destroyed => {
                    *control_flow = glutin::event_loop::ControlFlow::Exit;
                    return;
                }
                glutin::event::WindowEvent::KeyboardInput { input, .. } => {
                    if let Some(virtual_key) = input.virtual_keycode {
                        game_context.press_key(virtual_key);
                    }
                }
                glutin::event::WindowEvent::Focused(b) => {
                    game_context.set_focus(b);
                }
                glutin::event::WindowEvent::CursorMoved {
                    position: physical_position,
                    ..
                } => {
                    game_context.move_cursor(LogicalPosition::from_physical(
                        physical_position,
                        scale_factor,
                    ));
                }
                glutin::event::WindowEvent::MouseInput {
                    state,
                    button,
                    modifiers,
                    ..
                } => match state {
                    glutin::event::ElementState::Pressed => {
                        game_context.press_mouse(button, modifiers);
                    }
                    glutin::event::ElementState::Released => {
                        game_context.release_mouse(button, modifiers);
                    }
                },
                glutin::event::WindowEvent::ScaleFactorChanged {
                    scale_factor: new_scale_factor,
                    ..
                } => {
                    scale_factor = new_scale_factor;
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
