# Maze War

A Rust recreation of [Maze War](http://www.digibarn.com/history/04-VCF7-MazeWar/index.html) (1973) — one of the first networked first-person shooters.

## Overview

Maze War is a multiplayer FPS built in Rust using software ray casting and UDP networking. The game renders a first-person 3D view of a procedurally generated maze alongside a top-down minimap, with real-time multiplayer over a client-server architecture supporting 10+ simultaneous players.

## Architecture

Three crates in a single Cargo workspace:

| Crate | Responsibility |
|---|---|
| `common` | Shared types: maze grid, player state, packet protocol |
| `server` | UDP server, peer state management, game authority |
| `client` | Window, ray caster, minimap, HUD, network thread |

## Features

### Maze Generation
Mazes are procedurally generated using a two-step algorithm:
1. **Recursive Backtracker (DFS)** — produces a perfect maze with exactly one path between any two cells
2. **Braiding pass** — removes walls at dead ends with probability `braid_factor`, punching loops into the maze

Difficulty is controlled by `braid_factor`:

| Level | `braid_factor` | Character |
|---|---|---|
| 1 | 0.65 | Open corridors, easy navigation |
| 2 | 0.25 | Balanced layout, moderate dead ends |
| 3 | 0.00 | Perfect maze, maximum dead ends |

### Renderer
- DDA ray casting — one ray per screen column
- Perpendicular wall distance → column height: `screen_h / perp_dist`
- Distance-based wall shading
- Per-column Z-buffer for sprite depth testing
- Other players rendered as sprites, correctly clipped against the Z-buffer
- Target: sustained 50+ FPS

### Networking
- UDP client-server over `tokio`
- Server broadcasts `ServerSnapshot` (peer states + maze) each tick
- Client sends local state at ~20–30 Hz, renders at 60 Hz
- Peer state shared between network and render threads via `Arc<Mutex<PeerMap>>`
- Packet protocol serialized with `bincode`

```rust
enum Packet {
    Join    { name: String },
    State   { pos: Vec2, angle: f32, alive: bool },
    Shoot,
    ServerSnapshot { peers: Vec<PeerState>, maze: Vec<u8>, level: u8 },
}
```

## Getting Started

```bash
git clone https://github.com/D0ulo5/Maze-War
cd Maze-War

# Run the server
cargo run -p server

# Run a client (in a separate terminal)
cargo run -p client
```

You will be prompted for the server IP and a username (4–16 alphanumeric characters).

## Built With

- [Rust](https://www.rust-lang.org/)
- [`winit`](https://docs.rs/winit) — windowing
- [`pixels`](https://docs.rs/pixels) — raw pixel framebuffer
- [`tokio`](https://tokio.rs/) — async UDP networking
- [`bincode`](https://docs.rs/bincode) + [`serde`](https://serde.rs/) — packet serialization
- [`rand`](https://docs.rs/rand) — maze generation RNG

## References

- [Lode's Raycasting Tutorial](https://lodev.org/cgtutor/raycasting.html)
- [Maze War history](http://www.digibarn.com/history/04-VCF7-MazeWar/index.html)
