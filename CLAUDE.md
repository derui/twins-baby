# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

This is a Rust web application that combines Leptos (frontend framework) with Bevy (game engine) to create a web-based 3D application. The project demonstrates integration between Leptos for UI and Bevy for 3D rendering within a web canvas.

## Build and Development Commands

### Basic Commands
- `trunk serve` - Start development server with hot reload
- `trunk build` - Build for production
- `trunk build --release` - Build optimized release version

### Code Formatting and Linting
- `rustfmt --edition 2024 src/**/*.rs` - Format Rust code
- `leptosfmt src/**/*.rs` - Format Leptos components
- `pre-commit run --all-files` - Run all pre-commit hooks

### Testing
No specific test commands are configured in this project yet.

## Architecture

### Core Structure
- `src/main.rs` - Entry point that mounts the Leptos app
- `src/leptos_app.rs` - Main Leptos component with UI layout and Bevy canvas integration
- `src/bevy_app/` - Bevy application modules

### Bevy Integration
The project uses `leptos-bevy-canvas` to embed Bevy applications within Leptos components:

- `bevy_app/mod.rs` - Bevy app initialization with plugins and systems
- `bevy_app/setup.rs` - Scene setup (cubes, lighting, materials)
- `bevy_app/camera.rs` - Camera configuration with pan-orbit controls
- `bevy_app/pan_orbit.rs` - Pan-orbit camera controller implementation

### Key Integration Points
- The Bevy app is initialized in `init_bevy_app()` and embedded via `<BevyCanvas>` component
- Canvas is configured to render to `#bevy_canvas` element
- Bevy uses WebGPU backend for web rendering
- Pan-orbit camera system provides 3D navigation controls

## Technology Stack

### Frontend
- **Leptos 0.8.8** - Reactive web framework with CSR (Client-Side Rendering)
- **TailwindCSS 3.4.17** - Utility-first CSS framework

### 3D Engine
- **Bevy 0.16.1** - Game engine with these key features enabled:
  - `bevy_pbr` - Physically-based rendering
  - `bevy_picking` - 3D object interaction
  - `webgpu` - Web graphics backend
  - `bevy_mesh_picking_backend` - Mesh-based picking

### Build Tools
- **Trunk** - WASM web application bundler for Rust
- **Cargo** - Rust package manager
- **Nix** - Development environment management (flake.nix)

## Development Environment

The project uses Nix flakes for reproducible development environments. The flake provides:
- Rust toolchain with WASM target support
- Trunk for building and serving
- Pre-commit hooks for code formatting
- All necessary system dependencies for Bevy (graphics libraries, etc.)

To enter the development environment: `nix develop`

## Code Formatting

Pre-commit hooks are configured to automatically format code:
- `rustfmt` with edition 2024 for general Rust code
- `leptosfmt` specifically for Leptos component formatting

Both formatters run on `.rs` files to ensure consistent code style.

## Cargo Workspace Dependencies

This project uses Cargo workspace to manage shared dependencies across multiple crates. Common dependencies are defined in the root `Cargo.toml` under `[workspace.dependencies]`.

When adding dependencies to workspace member crates (`lib/cad-base`, `lib/solver`, etc.):
- **Always check** if the dependency is already defined in `[workspace.dependencies]` in the root `Cargo.toml`
- **If defined**, use `<dependency>.workspace = true` syntax instead of specifying the version directly
- **If not defined**, consider adding it to `[workspace.dependencies]` first if it will be used by multiple crates

Example usage in member crates:
```toml
[dependencies]
anyhow.workspace = true

[dev-dependencies]
approx.workspace = true
pretty_assertions.workspace = true
rstest.workspace = true
```

# Coding Preferences

## Testing
All test case must follow these styles:

- Use `AAA` Pattern, there are `Arrange`, `Act`, and `Assert`
  - Must add comment for each block
- When assertion for Rust's `Result`, avoid `is_ok` or `is_err` for assert generally.
- **Do not write overly engineering test** 
  - Always write effective test, such as use paratemerized test for patterns
- **Write more careful test cases for edge cases, MECE conditions**

### Libraries
- Use `pretty_assertions::assert_eq` instead of `std::assert_eq` in Rust test modules
- Import pretty_assertions at the top of test modules with `use pretty_assertions::assert_eq;`
- Use `approx::assert_relative_eq!` for floating point comparisons in tests
- Import approx at the top of test modules with `use approx::assert_relative_eq;`
