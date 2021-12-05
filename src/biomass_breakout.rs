pub mod simulation;

use crate::game;
use crate::game::Game;
use glium::glutin;
use glium::implement_vertex;
use glutin::dpi::{LogicalPosition, LogicalSize};

/// Holds the game state which will be needed for rendering, as well as any handles
/// to auxilliary utilities.
///
/// For instance, we will likely have a handle used to record game execution
/// for testing and debugging purposes. For now we only have fonts and
/// associated textures in here.
pub struct BiomassBreakout {
    pub state: State,
    pub text_system: game::font::TextSystem,
    pub font_texture: game::font::FontTexture,
}

#[derive(Debug)]
pub enum State {
    Menu,
}

#[derive(Copy, Clone)]
struct MenuVertex {
    position: [f32; 2],
}

fn menu_vertex(x: f32, y: f32) -> MenuVertex {
    MenuVertex { position: [x, y] }
}

implement_vertex!(MenuVertex, position);

#[derive(Debug)]
pub enum Message {}

impl Game for BiomassBreakout {
    type Message = Message;

    fn render(&self, display: &mut glium::Display) {
        use glium::Surface;

        let mut target = display.draw();
        target.clear_color(0.35, 0.53, 0.78, 0.0);
        match self.state {
            State::Menu => {
                let text = game::font::TextDisplay::new(
                    &self.text_system,
                    &self.font_texture,
                    "Biomass Breakout",
                );
                let text_width = text.get_width();
                let text_height = text.get_height();
                let (_w, _h) = display.get_framebuffer_dimensions();
                #[rustfmt::skip]
                let matrix: [[f32; 4]; 4] = (cgmath::Matrix4::from_translation(
                    cgmath::Vector3 { x: -0.5, y: 1.0 - 0.5 * text_height, z: 0.0f32 }
                ) * cgmath::Matrix4::from_scale(1.0 / text_width))
                .into();
                game::font::draw(
                    &text,
                    &self.text_system,
                    &mut target,
                    matrix,
                    (1.0, 1.0, 0.0, 1.0),
                )
                .unwrap();
                let shape = vec![
                    menu_vertex(-0.8, 1.0 - 0.5 * text_height - 0.1),
                    menu_vertex(0.8, 1.0 - 0.5 * text_height - 0.1),
                    menu_vertex(0.8, -1.0 + 0.5 * text_height - 0.1),
                    menu_vertex(-0.8, -1.0 + 0.5 * text_height - 0.1),
                    menu_vertex(-0.8, 1.0 - 0.5 * text_height - 0.1),
                ];
                let logo_box_vertex_buffer = glium::VertexBuffer::new(display, &shape).unwrap();
                let indices = glium::index::NoIndices(glium::index::PrimitiveType::TriangleStrip);
                let logo_box_vertex_shader_src = r#"
                    #version 140

                    in vec2 position;

                    void main() {
                        gl_Position = vec4(position, 0.0, 1.0);
                    }
                "#;

                let logo_box_fragment_shader_src = r#"
                    #version 140

                    out vec4 color;

                    void main() {
                        color = vec4(0.4, 0.8, 0.5, 1.0);
                    }
                "#;
                let logo_box_program = glium::Program::from_source(
                    display,
                    logo_box_vertex_shader_src,
                    logo_box_fragment_shader_src,
                    None,
                )
                .unwrap();
                target
                    .draw(
                        &logo_box_vertex_buffer,
                        &indices,
                        &logo_box_program,
                        &glium::uniforms::EmptyUniforms,
                        &Default::default(),
                    )
                    .unwrap();
            }
        }
        target.finish().expect("finish no work?");
    }

    fn receive(&mut self, _message: Self::Message) {}

    fn press_key(&mut self, _virtual_key: glutin::event::VirtualKeyCode) {}

    fn release_key(&mut self, _virtual_key: glutin::event::VirtualKeyCode) {}

    fn set_focus(&mut self, _focus: bool) {}

    fn move_cursor(&mut self, _new_position: LogicalPosition<f64>) {}

    fn press_mouse(&mut self, _button: glutin::event::MouseButton) {}

    fn release_mouse(&mut self, _button: glutin::event::MouseButton) {}

    fn change_modifiers(&mut self, _modifiers: glutin::event::ModifiersState) {}

    fn resize(&mut self, _new_size: LogicalSize<f64>) {}
}

#[cfg(test)]
mod test {}
