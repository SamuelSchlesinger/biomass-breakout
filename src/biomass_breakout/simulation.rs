use std::collections::{BTreeMap, BTreeSet};

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
    Starvation,
    /// Death by murder
    KilledBy(CreatureId),
    /// Death by self
    Suicide,
}

pub struct PopulationFitness {
    /// The result of an evaluation of the population's fitness
    fitness: BTreeMap<CreatureId, f64>,
}

/// A trait for encapsulating different strategies for modifying the population.
pub trait SelectionStrategy {
    /// Modify the population with the given fitness criterion, according to
    /// the strategy.
    fn select(self, population: &mut Population, fitness: PopulationFitness);
}
