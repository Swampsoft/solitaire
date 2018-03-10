# Shenzhen I/O Solitaire Clone

![screenshot](https://i.imgur.com/6d7PdiE.jpg)

_This project is not associated with Zachtronics or SHENZHEN I/O._

This is an implementation of the Solitaire mini-game included in the game SHENZHEN I/O. It comes with a set of custom 
made graphics and no audio, but can use the original game's assets if they are installed on the system.

## Installation

The game is written in [Rust](https://www.rust-lang.org/en-US/). If you do not have a rust toolchain on your system 
yet, [rustup](https://www.rustup.rs/) is the preferred way to change that. When the toolchain is ready build and run the
game with

    cargo run --release
    
If you want to use the original graphics from SHENZHEN I/O, the easiest way is to delete or rename the 
`resources/textures/` directory. Then the game will load assets from 
`~/.local/share/Steam/SteamApps/common/SHENZHEN IO/Content/`. This works only on Linux if SHENHZHEN I/O is  installed 
in Steam. If you are on Windows, or have SHENZHEN I/O installed in a different location, simply copy the following 
directories and files:
 
 1. `SHENZHEN IO/Content/textures/solitaire` -> `resources/textures/solitaire`
 2. `SHENZHEN IO/Content/textures/sounds/card_*.wav` -> `resources/sounds/`
 3. `SHENZHEN IO/Content/textures/music/Solitaire.ogg` -> `resources/music/`

## How to play

The original game comes with ingame instructions. Please refer to these for now.
