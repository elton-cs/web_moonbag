# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Build Commands

This project is primarily developed for web deployment using Bevy CLI:

**Primary Commands (Web-focused):**
- `bevy run web` - Run web development build (primary development command)
- `bevy run web --release` - Run web release build for production
- `bevy run` - Run native development build (for debugging when needed)
- `bevy run --release` - Run native release build

**Quality Checks:**
- `cargo test` - Run tests
- `cargo fmt` - Format code
- `cargo clippy` - Run lints
- `cargo doc --no-deps --all-features` - Check documentation

## Architecture Overview

This is a Bevy 0.16 game with modular plugin architecture supporting both native and web platforms:

### Core Systems
- **State Management**: Uses `Screen` enum for game flow (Splash → Title → Loading → Gameplay) - Currently disabled
- **Asset Loading**: Custom `LoadResource` trait with `ResourceHandles` for progress tracking - Currently disabled
- **Input System**: Centralized input recording in `RecordInput` system set via `AppSystems` ordering
- **Audio**: Background music and SFX with volume controls - Currently disabled
- **3D Rendering**: Uses 3D camera with directional and ambient lighting for background scene

### Module Organization
- `src/demo/` - Gameplay mechanics (player, movement, animation, level) - Currently disabled
- `src/screens/` - Game states and transitions - Currently disabled
- `src/menus/` - UI screens (main menu, pause, settings, credits) - Currently disabled
- `src/theme/` - UI theming and widgets - Currently disabled
- `src/dev_tools.rs` - Development utilities (conditional compilation) - Currently disabled
- `src/torii/` - Blockchain integration using Torii client for Dojo/Starknet (actively used)
- `src/background.rs` - 3D background rendering system with rotating starfield and moon model

### Key Patterns
- **ECS**: Component-based entities with system ordering via `AppSystems`
- **Plugins**: Each major feature as a separate plugin
- **Pausable Systems**: Systems respect `Pause` state
- **UI**: Theme-based with reusable widgets and interaction systems

### Platform Considerations
- Web builds disable asset meta checking to prevent panics
- Uses conditional compilation for platform-specific features
- Logging optimized out in release builds for performance
- Canvas fits to parent container for web deployment
- Uses 3D camera positioned at (0,0,5) looking toward origin

### Asset Configuration
- Images use nearest neighbor sampling for pixel-perfect rendering
- 3D models loaded from .glb files with scene extraction
- Unlit materials used for background to avoid lighting calculations

## Current Project State

**Active Plugins:**
- `background::plugin` - Renders 3D starfield background with rotating moon model using starbg.png texture and moon.glb
- `torii::plugin` - Blockchain integration with Dojo/Starknet entity streaming

**Disabled Plugins (commented out in main.rs):**
- Most game features are currently disabled during development
- To enable features, uncomment the relevant plugin in `src/main.rs`

## Development Notes

- Primary development target is web (`wasm32-unknown-unknown`)
- Default feature is `dev_native` with dynamic linking and file watching
- Web builds use `bevy run web` command with automatic profile selection
- Asset meta checking disabled for web builds to prevent panics
- Torii client requires Tokio runtime context (may cause panics if not properly configured)

## Dependencies

**Blockchain Integration:**
- `torii-client` and `torii-proto` for Dojo/Starknet integration
- `dojo-types` for blockchain type definitions
- `starknet` for Starknet network interaction

**Web Support:**
- `wasm-bindgen` and `wasm-bindgen-futures` for WebAssembly
- `web-sys` for browser API access