## Chip 8 Emulator

This is an implementation of a Chip-8 Emulator in Rust.

## Requirements

- Rust
- SDL2

```bash
brew install rust sdl2 sdl2_gfx
```

## Usage

```bash
# Export Homebrew Library path for SDL
export LIBRARY_PATH="$LIBRARY_PATH:$(brew --prefix)/lib"

cargo run <scale> <delay> <rom>

# For example
cargo run 20 1 Test-Opcodes.ch8
```