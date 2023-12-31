# Chip 8 Emulator

This is an implementation of a Chip-8 Emulator in Rust.

It is a direct port of the C++ implementation [here](https://gitlab.casey.sh/casey/chip8).

![](https://i.imgur.com/OXQ6fMs.png)

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

## References

- [starrhorne/chip8-rust](https://github.com/starrhorne/chip8-rust)
- [caseyscarborough/chip8-cpp](https://github.com/caseyscarborough/chip8-cpp)
