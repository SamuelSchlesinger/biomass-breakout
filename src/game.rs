mod gl_context;

use gl_context::GLContext;
use glium::glutin;
use glutin::dpi::{LogicalPosition, LogicalSize};

pub trait Game {
    fn render(&self, display: &mut glium::Display);

    fn press_key(&mut self, virtual_key: glium::glutin::event::VirtualKeyCode);

    fn set_focus(&mut self, focus: bool);

    fn move_cursor(&mut self, new_position: LogicalPosition<f64>);

    fn press_mouse(&mut self, button: glutin::event::MouseButton);

    fn release_mouse(&mut self, button: glutin::event::MouseButton);

    fn change_modifiers(&mut self, modifiers: glutin::event::ModifiersState);

    fn resize(&mut self, new_size: LogicalSize<f64>);

    fn release_key(&mut self, virtual_key: glium::glutin::event::VirtualKeyCode);

    // Thought: Go off into a thread and run
    // create_game. Until its done, display
    // some generic loading screen. That way,
    // loading screens are baked into the
    // "game engine".
    fn run<G: Game, F>(create_game: F) -> !
    where
        F: FnOnce() -> G,
        G: 'static,
    {
        let GLContext {
            event_loop,
            mut display,
            starting_scale_factor,
        } = GLContext::default();
        let mut game = create_game();
        let mut scale_factor = starting_scale_factor;
        event_loop.run(move |event, _event_loop_window_target, control_flow| {
            // Rendering
            game.render(&mut display);

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
                    }
                    glutin::event::WindowEvent::Destroyed => {
                        *control_flow = glutin::event_loop::ControlFlow::Exit;
                    }
                    glutin::event::WindowEvent::KeyboardInput { input, .. } => {
                        if let Some(virtual_key) = input.virtual_keycode {
                            match input.state {
                                glutin::event::ElementState::Pressed => game.press_key(virtual_key),
                                glutin::event::ElementState::Released => {
                                    game.release_key(virtual_key)
                                }
                            }
                            game.press_key(virtual_key);
                        }
                    }
                    glutin::event::WindowEvent::Focused(b) => {
                        game.set_focus(b);
                    }
                    glutin::event::WindowEvent::Resized(new_physical_size) => {
                        game.resize(LogicalSize::from_physical(new_physical_size, scale_factor));
                    }
                    glutin::event::WindowEvent::CursorMoved {
                        position: physical_position,
                        ..
                    } => {
                        game.move_cursor(LogicalPosition::from_physical(
                            physical_position,
                            scale_factor,
                        ));
                    }
                    glutin::event::WindowEvent::ModifiersChanged(new_modifiers) => {
                        game.change_modifiers(new_modifiers);
                    }
                    glutin::event::WindowEvent::MouseInput { state, button, .. } => match state {
                        glutin::event::ElementState::Pressed => {
                            game.press_mouse(button);
                        }
                        glutin::event::ElementState::Released => {
                            game.release_mouse(button);
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
}
