# Sand Simulation (Rust + macroquad)

A real-time, Noita-style falling-sand cellular automaton written in Rust.

![Sand Simulation Screenshot](images/image.png)

## Overview

This project is an interactive particle simulator where you can paint materials onto a grid and watch emergent behavior:

- **Sand** falls and piles up
- **Water** flows, spreads, and is displaced by denser materials
- **Stone**, **Wood**, **Fire**, and **Smoke** are defined in the data model (with partial/ongoing rule implementation)

The simulation is built around a simple, explicit CPU update loop with deterministic grid traversal and per-cell rules.

---

## Current Status

### Implemented
- Window setup with `macroquad`
- Grid-backed cellular automata core
- Bottom-to-top update order
- Alternating horizontal scan direction per row (to reduce directional bias)
- Density-based displacement helper (`can_displace`)
- Sand update rule
- Water update rule (including lateral spread logic)
- Mouse painting with brush radius
- Material switching via typed characters (AZERTY/QWERTY-friendly for current mapped keys)
- Rendering pipeline optimized to:
  - reuse a persistent texture
  - reuse a persistent RGBA byte buffer
  - update texture bytes each frame
- Pixel-art scaling via nearest-neighbor filtering

### In Progress / Planned
- Fire, Smoke, Wood full behavior ecosystem
- Pause, clear-grid, brush-size controls
- UI material picker panel
- Chunk/activity-based update optimization

---

## Tech Stack

- **Language:** Rust (stable, edition 2024)
- **Framework:** `macroquad` (`0.4.14`)
- **Simulation:** CPU-based cellular automata, single-threaded

---

## Project Structure

```sand_sim/src/main.rs#L1-200
mod config;
mod grid;
mod render;
mod rules;
```

High-level module responsibilities:

- `src/main.rs` — app entry point, game loop, input handling, wiring update + draw
- `src/config.rs` — core constants (grid size, cell size, FPS target, brush radius)
- `src/grid.rs` — `Grid`, `Cell`, `Material`, indexing helpers, paint/update loop, rendering data upload
- `src/rules.rs` — per-material simulation logic
- `src/render.rs` — drawing trait abstraction (`Texturable`)

---

## Configuration

Current defaults in `src/config.rs`:

- `GRID_WIDTH = 400`
- `GRID_HEIGHT = 300`
- `CELL_SIZE = 3`
- `TARGET_FPS = 60` (defined; frame limiting strategy can be expanded)
- `DEFAULT_BRUSH_R = 3`

Window size is derived from `GRID_* × CELL_SIZE`.

---

## Controls

### Mouse
- **Left click / hold:** paint selected material

### Keyboard (currently mapped)
- `&` or `1` → **Sand**
- `é` or `2` → **Water**

> Notes:
> - The project currently uses character input, which helps with non-QWERTY layouts (e.g. AZERTY).
> - Additional material keys are scaffolded/commented and can be enabled as rules are finalized.

---

## Build & Run

## Prerequisites
- Rust toolchain installed (`rustup`, `cargo`)

## Run
```sand_sim/README.md#L95-96
cargo run
```

## Build (release)
```sand_sim/README.md#L99-100
cargo build --release
```

---

## Simulation Design Highlights

- Grid stored as a flat `Vec<Cell>` in row-major order
- Update pass clears `updated` flags each frame
- Traversal is bottom-to-top to avoid multi-move “teleport” artifacts
- Row scan direction alternates to mitigate left/right bias
- Movement decisions use material density comparisons:
  - `can_displace(mover, target) -> density(mover) > density(target)`

---

## Rendering Notes

The renderer avoids expensive per-frame allocations by:

1. Keeping a persistent `texture_bytes: Vec<u8>`
2. Writing per-cell colors directly into that buffer each frame
3. Updating an existing `Texture2D` from bytes
4. Drawing scaled-up with nearest filtering

This is substantially more efficient than recreating textures every frame.

---

## Known Issues / Next Fixes

- Finish rule implementations for `Fire`, `Smoke`, `Wood`
- Introduce activity/chunk-based updates for larger-scale performance

---

## Inspiration

- Falling-sand / powder-toy style cellular simulations
- Noita-like emergent material interactions