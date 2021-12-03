# Biomass Breakout

## Backstory

An ark has crash landed on the failed colony of Vivemortis, which humans had
tried and failed to tame. As it crashes, the tanks of biomass along with the
engine's byproducts which were collected during transit leaked into the soil
and became the life which would occupy this planet. Some say that humans
survived and merged with the cosmic mud.

When the players are modifying the terrain as the game 

## Game Mechanics

As you come into the game, you will be presented with options to seed your gene
pool. There will be some set of genes which control the behavior, capabilities,
and appearance of your creatures, and your gene pool will consist of many different
creatures, though genetic variation will change over time.

The user will not directly control the creatures, and in that sense the game will
be passive and strategic, not active and tactical. The things which will be under
(perhaps partial) user control are:

1. The map for any particular engagement
2. The opponents you choose to face
3. The fitness function for selection
  a. Maximize/minimize
    1. Movement
    2. Food intake
    3. Amount of time in certain environment
  b. Combine N fitness functions by taking a weighted sum
  c. Boolean survival
  d. Predation statistics
    1. Maximimize/Minimize Quantity
    2. Boolean "did I predate?"
4. The selection mechanism
  a. Two parents
  b. One parent
  c. "Population Level Mixing"
  d. Maybe k-mixing is a generalization of two parents and population level mixing
  e. Totally random genes

The map will expand as the game continues, releasing more nutrients as it does, but
less than proportionally to the size increase such that scarcity increases over time,
and by the end of the game, much of the map should be inhospitable.
