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

There are three controls:
- Turn clockwise
  - While active:
     - Decreases the ship's energy
     - and gives a positive argular accelaration
- Turn clockwise
  - While active:
    - Decrease the ship's energy
    - and gives a negative argular accelaration
- Thrust
  - While active:
    - Decrease the ship's energy
    - and give an acceleration in the direction the ship is pointing

### Energy

### Thrust/Movement

### Hull

### Shields

### Torpedos

### Lasers

### Hyperspace

### Gravity Well

### Scoring, Death, and Respawning

### Pausing

### End of game

## Graphics


## Sound


## Optional Future Additions

- Four players
- Make game options selectable from the menu, things like:
  - no gravity well
  - no shields
  - etc.