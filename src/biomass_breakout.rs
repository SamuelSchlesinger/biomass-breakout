use crate::game::Game;
use glium::glutin;
use glutin::dpi::LogicalPosition;

/// Holds the game state, as well as any handles to auxilliary utilities.
/// For instance, we will likely have a handle used to record game execution
/// for testing and debugging purposes.
pub struct BiomassBreakout {
    state: State,
}

enum State {
    Loading,
}

impl Default for BiomassBreakout {
    fn default() -> Self {
        BiomassBreakout {
            state: State::Loading,
        }
    }
}

impl Game for BiomassBreakout {
    fn render(&self, display: &mut glium::Display) {
        use glium::Surface;
        let mut target = display.draw();
        target.clear_color(0.0, 0.0, 0.0, 0.0);
        match self.state {
            State::Loading => {}
        }
        target.finish().expect("finish no work?");
    }

    fn press_key(&mut self, virtual_key: glium::glutin::event::VirtualKeyCode) {}

    fn set_focus(&mut self, focus: bool) {}

    fn move_cursor(&mut self, new_position: LogicalPosition<f64>) {}

    fn press_mouse(&mut self, button: glutin::event::MouseButton) {}

    fn release_mouse(&mut self, button: glutin::event::MouseButton) {}

    fn change_modifiers(&mut self, modifiers: glutin::event::ModifiersState) {}
}
