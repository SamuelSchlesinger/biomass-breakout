mod biomass_breakout;
mod game;

use biomass_breakout::{BiomassBreakout, BiomassBreakoutContext};
use game::gl_context::GLContext;
use game::Game;

fn main() {
    let mut gl_context = GLContext::default();
    let font_system = game::font::TextSystem::new(&gl_context.display);
    let font_data = include_bytes!("/usr/share/fonts/truetype/dejavu/DejaVuSansMono.ttf");
    let font_texture = game::font::FontTexture::new(
        &gl_context.display,
        &font_data[..],
        70,
        game::font::FontTexture::ascii_character_list(),
    )
    .expect("Must be able to build font teture");
    <BiomassBreakout as Game>::run_with(gl_context, move || BiomassBreakout {
        state: biomass_breakout::State::Menu,
        font_system,
        font_texture,
    });
}
