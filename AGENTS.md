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
- `task lint` - Format and lint Rust code

### Testing
- `task test` - Run all tests in the workspace
- `task test -- -p <crate_name>` - Run tests for a specific crate only (recommended when editing a single crate)
- `task test -- --filter <test_name>` - Run specific tests matching the filter
- `task test -- --no-fail-fast` - Continue running all tests even after failures

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
- **Bevy 0.17.3** - Game engine with these key features enabled:
  - `bevy_pbr` - Physically-based rendering
  - `bevy_picking` - 3D object interaction
  - `webgpu` - Web graphics backend
  - `bevy_mesh_picking_backend` - Mesh-based picking

### Build Tools
- **Trunk** - WASM web application bundler for Rust
- **Cargo** - Rust package manager
- **Nix** - Development environment management (flake.nix)
- **Taskfile** - Organize tasks

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
- **Avoid accessing internal state in tests as much as possible**
  - Test behavior through public APIs rather than inspecting private fields
  - Focus on observable outcomes and side effects
  - Only access internal state when absolutely necessary for validation
- **Do not test simple getters**
  - Skip tests for trivial getter methods that only return a field value without any logic
  - Example: `pub fn name(&self) -> &str { &self.name }` does not need a test
  - Focus testing efforts on methods with actual behavior and logic
- When assertion for Rust's `Result`, avoid `is_ok` or `is_err` for assert generally.
- **Do not write overly engineering test**
  - Always write effective test, such as use paratemerized test for patterns
- **Write more careful test cases for edge cases, MECE conditions**
- **When large number of test cases (> 10), split it to dedicated test module**
  - if the module was a single file, make module directory and tests module.
  - E.g. When `foo.rs` has large tests, split it as `foo/mod.rs` and `foo/tests.rs`, and all test cases into `tests.rs`

### Libraries
- Use `pretty_assertions::assert_eq` instead of `std::assert_eq` in Rust test modules
- Import pretty_assertions at the top of test modules with `use pretty_assertions::assert_eq;`
- Use `approx::assert_relative_eq!` for floating point comparisons in tests
- Import approx at the top of test modules with `use approx::assert_relative_eq;`

# Commit convention

All commit must follow these styles:

- Follow conventional commit
  - We use `feat` , `refactor`, `fix`, `test` , `style`, `perf`, `chore` 
- Should add scope with Rust crate name when possible
