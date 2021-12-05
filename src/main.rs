mod biomass_breakout;
mod game;

use biomass_breakout::BiomassBreakout;
use game::gl_context::GLContext;
use game::Game;

fn main() {
    let gl_context = GLContext::default();
    let text_system = game::font::TextSystem::new(&gl_context.display);
    let font_data = include_bytes!("../assets/font.ttf");
    let font_texture = game::font::FontTexture::new(
        &gl_context.display,
        &font_data[..],
        70,
        game::font::FontTexture::ascii_character_list(),
    )
    .expect("Must be able to build font teture");
    let event_loop_proxy = gl_context.event_loop.create_proxy();
    std::thread::spawn(move || {
        // For now, we simply start a simulation as the game begins,
        // and that communicates with the game to display the game state.
        biomass_breakout::simulation::begin(|event| {
            event_loop_proxy.send_event(event).expect("c'mon")
        });
    });
    <BiomassBreakout as Game>::run_with(gl_context, move || BiomassBreakout {
        state: biomass_breakout::State::Menu,
        text_system,
        font_texture,
    });
}
