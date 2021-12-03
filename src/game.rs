use glium::glutin;
use glutin::dpi::LogicalPosition;

/// Holds the game state, as well as any handles to auxilliary utilities.
/// For instance, we will likely have a handle used to record game execution
/// for testing and debugging purposes.
pub struct GameContext {
    state: GameState,
}

enum GameState {
    Loading,
}

impl GameContext {
    pub fn new() -> Self {
        GameContext {
            state: GameState::Loading,
        }
    }

    pub fn render(&self, display: &mut glium::Display) {
        use glium::Surface;
        let mut target = display.draw();
        target.clear_color(0.0, 0.0, 0.0, 0.0);
        match self.state {
            GameState::Loading => {}
        }
        target.finish().expect("finish no work?");
    }

    pub fn press_key(&mut self, virtual_key: glium::glutin::event::VirtualKeyCode) {}

    pub fn set_focus(&mut self, focus: bool) {}

    pub fn move_cursor(&mut self, new_position: LogicalPosition<f64>) {}

    pub fn press_mouse(
        &mut self,
        button: glutin::event::MouseButton,
        modifiers: glutin::event::ModifiersState,
    ) {
    }

    pub fn release_mouse(
        &mut self,
        button: glutin::event::MouseButton,
        modifiers: glutin::event::ModifiersState,
    ) {
    }
}
