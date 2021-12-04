mod biomass_breakout;
mod game;

use biomass_breakout::BiomassBreakout;
use game::Game;

fn main() {
    <BiomassBreakout as Game>::run(BiomassBreakout::default);
}
