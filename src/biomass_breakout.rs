mod simulation;

use crate::game;
use crate::game::Game;
use glium::glutin;
use glutin::dpi::{LogicalPosition, LogicalSize};
use std::collections::BTreeMap;

/// Holds the game state, as well as any handles to auxilliary utilities.
/// For instance, we will likely have a handle used to record game execution
/// for testing and debugging purposes.
pub struct BiomassBreakout {
    pub state: State,
    pub font_system: game::font::TextSystem,
    pub font_texture: game::font::FontTexture,
}

#[derive(Debug)]
pub enum State {
    Menu,
}

impl BiomassBreakout {
    fn flip_state(&mut self) {
        let new_state = match self.state {
            State::Menu => State::Menu,
        };
        self.state = new_state;
    }
}

#[derive(Debug)]
pub struct BiomassBreakoutContext {
    pub fonts: BTreeMap<String, rusttype::Font<'static>>,
}

impl Game for BiomassBreakout {
    fn render(&self, display: &mut glium::Display) {
        use glium::Surface;

        let mut target = display.draw();
        target.clear_color(0.35, 0.53, 0.78, 0.0);
        match self.state {
            State::Menu => {
                let text = game::font::TextDisplay::new(
                    &self.font_system,
                    &self.font_texture,
                    "Biomass Breakout",
                );
                let text_width = text.get_width();
                let text_height = text.get_height();
                let (w, h) = display.get_framebuffer_dimensions();
                #[rustfmt::skip]
                let matrix: [[f32; 4]; 4] = (cgmath::Matrix4::from_translation(
                    cgmath::Vector3 { x: -0.50, y: 1.0 - 0.5 * text_height, z: 0.0f32 }
                ) * cgmath::Matrix4::from_scale(1.0 / text_width))
                .into();
                game::font::draw(
                    &text,
                    &self.font_system,
                    &mut target,
                    matrix,
                    (1.0, 1.0, 0.0, 1.0),
                )
                .unwrap();
            }
        }
        target.finish().expect("finish no work?");
    }

    fn press_key(&mut self, _virtual_key: glutin::event::VirtualKeyCode) {
        self.flip_state();
    }

    fn release_key(&mut self, _virtual_key: glutin::event::VirtualKeyCode) {}

    fn set_focus(&mut self, _focus: bool) {}

    fn move_cursor(&mut self, _new_position: LogicalPosition<f64>) {}

    fn press_mouse(&mut self, _button: glutin::event::MouseButton) {}

    fn release_mouse(&mut self, _button: glutin::event::MouseButton) {}

    fn change_modifiers(&mut self, _modifiers: glutin::event::ModifiersState) {}

    fn resize(&mut self, _new_size: LogicalSize<f64>) {}
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn flip_state_works() {}
}
