# Requirements

## Overview

Spacewar! is a game of two space ships dogfighting around a gravity well with newtonian physics based movement.  When ships cross off screen, they reemerge on the other side.

This version will be a reimplementation of spacewar! with spice thrown in to make the game play more complex and be more graphically appealing.

## Workflow

The game should start at a menu, then from that menu the players can launch the desired game variant.

## Menu

Game should have:

- A menu item to select a new game
  - New game, you can select either 2 local players
  - or 1 local and 1 AI player
- A menu item to view the controls
- A menu item to view help and the rules and mechanics of the game

## Gameplay

### Movement

The ships retain both momentum and argular momentum if left alone.

### Controls

Each player has three controls:
- Turn clockwise
  - While active:
    - Decreases the ship's energy
    - and gives a positive argular accelaration
    - if there isn't enough ship energy left, do nothing
- Turn clockwise
  - While active:
    - Decrease the ship's energy
    - and gives a negative argular accelaration
    - if there isn't enough ship energy left, do nothing
- Thrust
  - While active:
    - Decrease the ship's energy
    - and give an acceleration in the direction the ship is pointing
    - if there isn't enough ship energy left, do nothing

Each player has the following actions:
- Fire a torpedo (See: Torpedos)
- Fire laser (See: Lasers)
- Engage hyperspace (see: Hyperspace)
- Recharge shields (see: Shields)


### Energy

- Ship has a a pool of energy with a maxium amount
- When is isn't full, it slowly recharges until full again

### Hull

- Ship have a fixed maxium hull points
- When the ship's hull points reach zero, the ship is destructed
- Hull points are not recovered

### Shields

- The ship has a fixed number of shield points
- The shields can be recharged by subtracting a large amount from the ship's energy (if there is enough remaining)

### Torpedos

- Torpedos can be fired either ship for the cost of a small amount of energy
- Torpedos start with a velocity of the ship that fired it plus a forward amount
- Torpedos accelerate forward briefly after being fired
- If a torpedos collides with a ship (any ship) is explodes and does damage
- If the ship has shields remaining and sufficient to still have shields after the damage is done, a small amount of damage is done to the shields
- If the ship has shields remaining, but insufficient to still have shields after the damage is done, damage it calculated with:
  - Shields reduced to zero
  - Hull damaged by "amount of hull damage done by torpedo minus shields that were remaining
- If the ship's has no shields, a large amount of damage is done to the hull

### Lasers

### Hyperspace

### Gravity Well

### Scoring, Death, and Respawning

## Pausing

## End of game

## Graphics

## Sound

## Optional Future Additions

- Four players
- Make game options selectable from the menu, things like:
  - no gravity well
  - no shields
  - etc.
