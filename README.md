# Spacewars

## Overview

I wanted to learn how to use the Amethyst Game Engine and improve my rust, so I decided to implement a game that I fondly remember from my childhood: **Spacewar!**.

**Spacewar!** has the honour of also being one of the very first computer game ever written, so it seems like a logical place to start when learning to use a game engine.

![Preview of gameplay](https://github.com/spotzero/spacewars/raw/master/preview.gif "Preview")

## Credits

Programming, art and graphics, and sound by David Pascoe-Deslauriers (@spotzero)

Built with the Amethyst Game Engine and the Rust programming language.

Music Attibution:
"Ignition, Set, GO!" by Bomb Boy is licensed under a Attribution-Noncommercial-Share Alike 3.0 United States License.
"Five Nine Seven Eight" by Virt is licensed under a Attribution-Noncommercial-Share Alike 3.0 United States License.
"Endorphemeral" by Zabutom is licensed under a Attribution-Noncommercial-Share Alike 3.0 United States License.
"From the Dunes" by Synapsis is licensed under a Creative Commons Attribution-NonCommercial-NoDerivatives 4.0 License.

## Building

### Linux

On Ubuntu, you need at least:

```
sudo apt install gcc pkg-config openssl libasound2-dev cmake build-essential python3 libfreetype6-dev libexpat1-dev libxcb-composite0-dev libssl-dev libx11-dev libfontconfig1-dev libsdl1.2-dev
```

To build the AppImage release, you need "cargo make", linuxdeploy, and appimagetool.  Then run:

```
cargo make release-linux
```

### Windows

Follow the directions for a SDL 2.0 to the project for Windows: https://github.com/Rust-SDL2/rust-sdl2

Then run:

```
cargo make release-windows
```

## License

This software is licensed under the GNU General Public License v3.0.  See LICENSE.txt for the full restrictions.
