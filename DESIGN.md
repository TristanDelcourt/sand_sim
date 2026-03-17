# Particle Simulator — Design Document

> A Noita-style falling-sand cellular automaton in Rust, built for two.

---

## Overview

A real-time interactive particle simulator where different materials obey physical rules and interact with each other. Sand falls and piles, water flows, fire spreads and burns, smoke rises and fades. The user paints materials onto the grid with a mouse brush and watches the simulation play out.

The goal is to build something that feels alive and emergent while remaining a well-structured Rust learning project. Complexity is layered across milestones so a beginner contributor can be productive from day one.

---

## Tech Stack

| Concern | Choice | Reason |
|---|---|---|
| Window + input | `macroquad` | Zero boilerplate, immediate-mode drawing, simple texture upload API |
| Simulation | Pure CPU, single-threaded to start | Easier to reason about update order; parallelism is a later milestone |
| Language | Rust (stable) | The whole point |

No ECS, no physics engine, no GPU compute. Keep the dependency tree minimal.

---

## Repository Structure

```
src/
  main.rs          — window init, main loop
  grid.rs          — Grid struct, indexing helpers, update dispatch
  cell.rs          — Cell struct, Material enum
  rules.rs         — one update function per material
  render.rs        — pixel buffer construction and texture upload
  input.rs         — mouse → grid coords, brush, material selection
  ui.rs            — material picker panel (added at milestone 5)
```

---

## Core Data Model

### `Material` enum

```
Air, Sand, Water, Stone, Wood, Fire, Smoke
```

Each material has two associated constants (defined in a `properties` function or a lookup table):
- **density** (`u8`) — determines what sinks through what; higher sinks through lower
- **flammable** (`bool`) — whether Fire can ignite it

### `Cell` struct

```
material:  Material
updated:   bool      // has this cell been processed this tick?
lifetime:  u8        // general-purpose counter (fire burn time, smoke fade)
```

Keep `Cell` small. The entire grid lives in a flat `Vec<Cell>` of size `width × height`, row-major (`index = y * width + x`).

### `Grid` struct

Owns the `Vec<Cell>` plus `width` and `height`. Exposes:
- `get(x, y) -> Cell`
- `set(x, y, cell)`
- `in_bounds(x, y) -> bool`
- `swap(x1, y1, x2, y2)` — the most-used primitive in rules

---

## Update Loop

Each frame:

1. Clear the `updated` flag on every cell (one pass over the `Vec`)
2. Iterate the grid **bottom-to-top** (row `height-1` down to `0`)
3. Within each row, alternate scan direction every frame (left→right on even frames, right→left on odd frames)
4. For each cell: if `!cell.updated`, call the rule function for its material
5. Rule functions set `updated = true` on any cell they move or transform

**Why bottom-to-top?** A falling particle moves downward. If you scan top-to-bottom and move it down, you then encounter it again at its new position and move it again — it teleports. Bottom-to-top means a particle lands in an already-visited cell and stays put until next frame.

**Why alternate scan direction?** Sand that can fall diagonally will always prefer one side if you always scan the same direction, creating an unnatural lean. Alternating breaks this symmetry.

---

## Material Rules

All rules live in `rules.rs`. Each rule receives a mutable reference to the `Grid` and the `(x, y)` coordinates of the cell being processed. Rules **only move or transform cells by calling `grid.swap()` or `grid.set()`** — no direct field mutation outside the grid abstraction.

### Density-based swapping

Before any directional logic, define a helper:

```
can_displace(mover: Material, target: Material) -> bool {
    density(mover) > density(target)
}
```

This single check covers: sand sinking through water, water sinking through air, smoke rising through air. Use it everywhere instead of hardcoding material pairs.

### Rules per material

**Stone**
Does nothing. No movement, no transformation. Exists as an immovable obstacle.

**Sand**
1. Try to move to `(x, y+1)` — if `can_displace(Sand, target)`, swap and return
2. Randomly choose to try `(x-1, y+1)` or `(x+1, y+1)` first (then the other)
3. If either diagonal is displaceable, swap and return
4. Otherwise, do nothing — the sand has settled

**Water**
1. Try to move to `(x, y+1)` — if displaceable, swap and return
2. Try diagonals `(x±1, y+1)` as with sand
3. If still stuck, try to spread horizontally: pick a random spread distance N (1–4), try `(x+1, y)` through `(x+N, y)` and `(x-1, y)` through `(x-N, y)`, swapping with the first Air cell found on either side

**Fire**
1. Decrement `lifetime`; if it reaches 0, transform to `Smoke` with a fresh lifetime and return
2. Check all 4 orthogonal neighbors: if any neighbor is `Wood` or another flammable material, set it to `Fire` with a random starting lifetime
3. Try to move to `(x, y-1)` — fire rises; if that cell is Air, swap

**Smoke**
1. Decrement `lifetime`; if it reaches 0, transform to `Air` and return
2. Try to move to `(x, y-1)` — if Air, swap
3. Occasionally (random 1-in-3 chance) try a random horizontal drift

**Wood**
Does nothing on its own. Only becomes `Fire` when a neighbor Fire rule ignites it.

---

## Rendering

The renderer builds a pixel buffer (`Vec<u8>`, RGBA, `width × height × 4` bytes) by iterating every cell and mapping its material to a color. Upload this buffer to a `macroquad` texture each frame and draw it fullscreen.

**Color palette** (suggested starting values):

| Material | Color |
|---|---|
| Air | `#1a1a2e` (dark background) |
| Sand | `#c2a35e` |
| Water | `#3a7bd5` with slight alpha |
| Stone | `#888888` |
| Wood | `#7a4f2d` |
| Fire | randomized between `#ff4500` and `#ff8c00` per cell per frame |
| Smoke | `#888888` with fading alpha based on remaining `lifetime` |

Fire and smoke benefit from per-cell color variation driven by `lifetime` — it makes them feel animated even without particle movement.

---

## Input Handling

Convert mouse position to grid coordinates: `grid_x = mouse_x / cell_pixel_size` (same for y). Clamp to bounds.

On left mouse button held: paint the currently selected material in a circular brush of radius R, overwriting whatever is there (except Stone if you want it to be permanent).

**Controls (initial, keyboard-driven):**

| Key | Action |
|---|---|
| `1` | Select Sand |
| `2` | Select Water |
| `3` | Select Stone |
| `4` | Select Wood |
| `5` | Select Fire |
| `[` / `]` | Decrease / increase brush radius |
| `Space` | Pause / unpause simulation |
| `C` | Clear grid |

---

## Work Split

### Tristan owns:
- `grid.rs` — data structure, indexing, swap primitive, update loop correctness
- `render.rs` — pixel buffer construction, texture upload, color palette
- `main.rs` — window init, frame loop, wiring everything together
- Performance work when it becomes relevant (dirty rectangles, etc.)
- Complex material rules (water spreading, fire propagation)

### Teammate owns:
- `rules.rs` — individual material rule implementations (start with Sand and Stone, progress to Water, then Fire/Smoke/Wood)
- `input.rs` — mouse position → grid coords, brush painting, keyboard material selection
- `ui.rs` (milestone 5) — material picker panel using macroquad drawing primitives

This split means the teammate can work entirely in `rules.rs` and `input.rs` without needing to understand the grid internals deeply at first.

---

## Milestones

### Milestone 1 — Skeleton
Window opens. A blank grid renders at 400×300 cells with each cell displayed as a 2×2 pixel square (800×600 window). The main loop runs at 60fps. Nothing moves. The pixel buffer upload and texture draw work correctly.

### Milestone 2 — Falling Sand
Sand falls and piles up correctly. The update order (bottom-to-top, alternating scan) is implemented and feels right. No directional bias. Stone exists as a static obstacle you can draw.

### Milestone 3 — Water and Density
Water flows and spreads. The density system is in place so Sand sinks through Water. The `can_displace` helper is used everywhere instead of hardcoded material checks.

### Milestone 4 — Fire Ecosystem
Wood, Fire, and Smoke all work. Fire spreads to adjacent Wood cells, burns out into Smoke, Smoke rises and fades. A scene of Wood towers set on fire should look believable and emergent.

### Milestone 5 — UI Upgrade
A side panel (fixed pixel width on the right side of the window) shows clickable material swatches drawn with macroquad's shape primitives. Brush size is shown and adjustable via mouse wheel. The currently selected material is highlighted. The simulation viewport shrinks to accommodate the panel.

### Milestone 6 — Performance: Dirty Rectangles
Track which grid regions were active last frame using a coarse chunk grid (e.g. 16×16 cell chunks). Only update and re-render chunks that had activity. This allows scaling to 800×600 cell grids without slowdown.

### Stretch Goals (pick any)
- Temperature system: cells have a temperature value; fire raises it, water lowers it; ice melts, water boils to steam
- Steam: a high-temperature Water variant that rises like Smoke
- Acid: dissolves Sand and Stone over time
- Gunpowder: static like Sand, ignites to a fast-spreading Fire on contact
- Oil: floats on Water (density below Water, above Air), burns intensely
- Save/load grid state to a file

---

## Configuration Constants

Define these at the top of `main.rs` or in a `config.rs`:

```
GRID_WIDTH:      400
GRID_HEIGHT:     300
CELL_SIZE:       2        // pixels per cell
TARGET_FPS:      60
DEFAULT_BRUSH_R: 3
```

These should be the only place you hardcode dimensions. Everything else derives from them.

---

## Key Gotchas

**The double-update bug.** If you forget to check and set the `updated` flag, a particle can be moved twice in one frame and appear to teleport. Always check `!cell.updated` before processing and mark both the source and destination as updated after a swap.

**Borrowing the grid in rules.** Rust will complain if you try to hold a reference to a cell while also mutating the grid. The pattern that works: copy the cell value out (`let cell = grid.get(x, y)`), do your logic on the copy, then call `grid.set()` or `grid.swap()`. Don't hold references across mutation calls.

**Diagonal preference.** Even with alternating scan direction, if your diagonal fallback always tries left before right, you'll see a subtle leftward lean in sand piles. Randomize which diagonal to try first each time a cell is evaluated.

**macroquad texture upload.** You need to call `update_texture` (not recreate the texture) each frame for performance. Create the texture once at startup, update its pixel data each frame.
