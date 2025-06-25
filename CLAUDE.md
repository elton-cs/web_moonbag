# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Build Commands

This project uses Bevy CLI for building and running:

- `bevy run` - Run native development build
- `bevy run --release` - Run native release build
- `bevy run web` - Run web development build  
- `bevy run web --release` - Run web release build

For testing and quality checks:
- `cargo test` - Run tests
- `cargo fmt` - Format code
- `cargo clippy` - Run lints
- `cargo doc --no-deps --all-features` - Check documentation

## Architecture Overview

This is a Bevy 0.16 game with modular plugin architecture supporting both native and web platforms:

### Core Systems
- **State Management**: Uses `Screen` enum for game flow (Splash → Title → Loading → Gameplay)
- **Asset Loading**: Custom `LoadResource` trait with `ResourceHandles` for progress tracking
- **Input System**: Centralized input recording in `RecordInput` system set
- **Audio**: Background music and SFX with volume controls

### Module Organization
- `src/demo/` - Gameplay mechanics (player, movement, animation, level)
- `src/screens/` - Game states and transitions
- `src/menus/` - UI screens (main menu, pause, settings, credits)
- `src/theme/` - UI theming and widgets
- `src/dev_tools.rs` - Development utilities (conditional compilation)

### Key Patterns
- **ECS**: Component-based entities with system ordering via `AppSystems`
- **Plugins**: Each major feature as a separate plugin
- **Pausable Systems**: Systems respect `Pause` state
- **UI**: Theme-based with reusable widgets and interaction systems

### Platform Considerations
- Web builds disable asset meta checking to prevent panics
- Uses conditional compilation for platform-specific features
- Logging optimized out in release builds for performance

## Development Notes

- Default feature is `dev_native` with dynamic linking and file watching
- Fast linker configs available in `.cargo/config_fast_builds.toml`
- Linux requires: `libasound2-dev`, `libudev-dev`, `libwayland-dev`
- Web target: `wasm32-unknown-unknown`