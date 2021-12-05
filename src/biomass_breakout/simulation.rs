/*!
A module which only need concern itself with the internal state of a
particular simulation, sending updates relevant to rendering about it
to the game engine, which will keep a data structure read optimized for
rendering.
*/

use crate::biomass_breakout;
use crate::game;
use biomass_breakout::BiomassBreakout;
use game::Game;
use std::collections::{BTreeMap, BTreeSet};

pub fn begin<F>(_send_event: F)
where
    F: FnMut(<BiomassBreakout as Game>::Message),
{
}

#[derive(Debug, PartialOrd, Ord, PartialEq, Eq)]
pub struct Population {
    /// The creatures in the population.
    creatures: BTreeMap<CreatureId, Creature>,
}

/// The static representation of any particular creature.
#[derive(Debug, PartialOrd, Ord, PartialEq, Eq)]
pub struct Creature {
    /// The primary value of any given creature is its genome.
    genome: Genome,
    /// Users can name their creatures
    name: Option<String>,
    /// Creatures have parents
    parents: BTreeSet<CreatureId>,
}

/// A creature is uniquely identified by an ID.
#[derive(Debug, PartialOrd, Ord, PartialEq, Eq)]
pub struct CreatureId(u64);

/// The genetic instructions for creatures
#[derive(Debug, PartialOrd, Ord, PartialEq, Eq)]
pub struct Genome {
    /// A genome, when it comes down to it, is just a
    /// page of bytes.
    bytes: [u8; 512],
}

/// Represents the cause of death of a creature
#[derive(Debug, PartialOrd, Ord, PartialEq, Eq)]
pub enum Obituary {
    /// Death by lack of food
    _Starvation,
    /// Death by murder
    _KilledBy(CreatureId),
    /// Death by self
    _Suicide,
}

#[derive(Debug, PartialOrd, PartialEq)]
pub struct PopulationFitness {
    /// The result of an evaluation of the population's fitness
    fitness: BTreeMap<CreatureId, f64>,
}

/// A trait for encapsulating different strategies for modifying the population.
pub trait SelectionStrategy {
    /// Modify the population with the given fitness criterion, according to
    /// the strategy.
    ///
    /// TODO(sam) We probably want to parameterize this on a source of
    /// randomness in order to not be stuck with the system generator?
    fn select(self, population: &mut Population, fitness: PopulationFitness);
}

#[derive(Debug, PartialOrd, PartialEq)]
pub struct Position {
    x: f64,
    y: f64,
}

#[derive(Debug, PartialOrd, PartialEq)]
pub struct Direction {
    angle: f64,
}

#[derive(Debug, PartialOrd, PartialEq)]
pub struct ActiveCreature {
    id: CreatureId,
    health: u64,
    position: Position,
    heading: Direction,
    metrics: BTreeMap<String, u64>,
}

trait FitnessFunction {
    fn fitness(self, active_creature: &ActiveCreature) -> f64;
}
