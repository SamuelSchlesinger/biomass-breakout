mod biomass_breakout;
mod game;

use biomass_breakout::BiomassBreakout;
use game::run;

fn main() {
    run(|| BiomassBreakout::default());
}
