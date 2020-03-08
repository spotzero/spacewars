# Game Design

This is my intial game design document.  It is a development starting point and **not** a reflection of the final game or product.

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
- Fire missle (See: Missles)
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
- If a torpedos collides with any ship, it explodes and does explosive damage.

### Lasers

- Lasers fire in a 30' arc  in front of the ship
- Lasers instantly hit the closest enemy ship in that arc
- While lasers are firing, they do energy damage
- Lasers consume energy while firing
- Lasers over-heat and cannot be continously fired

### Hyperspace

- Hyperspace jumps the ship to a random location on the map with the momentum preserved
- Hyperspace takes a large amount of energy
- When hyperspace is activated, there is a brief time before the jump where the ship is not controllable

### Missles

- Players hold down the missle button to "charge it"
- "Charging" a missle transfers energy from the ship's reverse to the missle
- Once released, the missle is fired.
- Fired missles track and attempt to intercept the closest target
- The amount the missle was charged determines how long the missle will track the target
- The missle explodes, causing explosive damage, when it is as close as possible to the target
  - If the missle is within damage radius, and the missle is further away from the target than the last frame, explode.
  - If the missle runs out of energy.

### Energy damage

- Energy damage drops off the further away the target is from the source
- Energy damage does 2x damage to shields
- If the ship has shields remaining and sufficient to still have shields after the damage is done, the calculated amount of damage is done to the shields, times 2.
- If the ship has shields remaining, but insufficient to still have shields after the damage is done, damage it calculated with:
  - Shields reduced to zero
  - Hull damaged by ("amount of damage done by energy" - "shields that were remaining")
- If the ship's has no shields, 1x damage to hull.

### Explosive damage

- Explosion damage is a circle area that takes instant damage, which does less damage the further away from the center the target is from the explosion.
- Explosion damage does 2x damage to hull versus shields.
- If the ship has shields remaining and sufficient to still have shields after the damage is done, the calculated amount of damage is done to the shields
- If the ship has shields remaining, but insufficient to still have shields after the damage is done, damage it calculated with:
  - Shields reduced to zero
  - Hull damaged by  x x("amount of damage done by explosion" - "shields that were remaining")
- If the ship's has no shields, 2x damage to hull.


### Gravity Well

- Gravity well is a circle in the center of the map
- It has a point gravitational pull that affects all game objects

### Scoring, Death, and Respawning

- Once a ships hull is reduced to zero, the ship in destroyed
- The player that dies gets 1 point added to their "deaths" count.
- If the play has fewer than 5 deaths: 3 seconds later, the ship respawns
- Otherwise, they do not respawn.

## Pausing

- Game can be paused at any time
- Pause screen should say "Paused" and play the battletoads pause drums.

## End of game

- When only one player remains (and none are due to respawn), the game ends and an victory screen is shown.
- Victory screen shows the winner and lists stats about the game
- The winner is the last surviving player

## Graphics

- Spawning should looking like the ship is hyperspacing into the map

## Optional Future Additions

- Four players
- Make game options selectable from the menu, things like:
  - no gravity well
  - no shields
  - etc.
