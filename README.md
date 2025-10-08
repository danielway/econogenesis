# Econogenesis

A terminal-based real-time economic simulator spanning from individual rooms to galactic scales.

[![Rust](https://img.shields.io/badge/rust-1.80%2B-orange.svg)](https://www.rust-lang.org)
[![License](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE)
[![Status](https://img.shields.io/badge/status-in%20development-yellow.svg)]()

## Overview

Econogenesis is a multi-scale economic simulation built in Rust. Navigate across 6 hierarchical zoom levels while observing how economic systems emerge from simple rules.

**Core features:**
- Multi-scale navigation from rooms to galaxies
- Real-time simulation with variable time control
- Procedurally generated universe
- Economic systems with markets, resources, and autonomous agents

## Current Status

### Foundation Complete

Core infrastructure complete with 18 passing tests:

| Component | Description |
|-----------|-------------|
| Rendering System | Terminal-based canvas, 30-60 FPS |
| Time Control | Play/pause, variable speed (0.1x to 50x) |
| Zoom Levels | 6-level hierarchy (Room to Galaxy) |
| Game Loop | Input, update, render architecture |
| Input System | Keyboard controls with help overlay |
| World State | Multi-scale entity system |

### In Development

Currently building navigation and procedural generation:
- Arrow key navigation at each zoom level
- Procedural galaxy generation (100-1000 star systems)
- Camera system with viewport management
- Enter/exit entities to traverse zoom levels

See [docs/navigation-and-maps-plan.md](docs/navigation-and-maps-plan.md) for details.

## Features

### Multi-Scale Zoom System
Six hierarchical zoom levels:
- **Galaxy** - Thousands of star systems
- **Solar System** - Planets orbiting stars
- **Planet** - Continental surfaces with terrain
- **Region** - Named areas with distinct characteristics
- **Local Area** - Buildings and structures
- **Room** - Individual interiors

### Time Control
- Play/pause simulation
- Speed control: 0.1x to 50x (8 preset speeds)
- Real-time FPS counter
- Simulation time tracking

### User Interface
- Terminal-based rendering via tty-interface
- Dynamic viewport with auto-clear
- Help overlay (H/?) showing all controls
- Position and location tracking
- 30-60 FPS performance

### Planned

Navigation system (in progress):
- Arrow key movement at each zoom level
- Enter key to zoom into entities
- Procedural universe generation
- Camera with viewport management
- Map rendering with ASCII/Unicode

Economic simulation:
- Resource systems with scarcity
- Autonomous agent AI
- Markets with price discovery
- Production chains
- Trade routes

## Getting Started

### Requirements

- Rust 1.80+ (2024 edition)
- Unicode-capable terminal
- Linux, macOS, or Windows (WSL recommended)

### Installation

```bash
git clone https://github.com/yourusername/econogenesis.git
cd econogenesis
cargo build --release
```

### Run

```bash
cargo run --release
```

### Controls

```
SPACE      Play/Pause
+/=        Increase speed
-/_        Decrease speed
Z          Zoom in
X          Zoom out
H/?        Help overlay
Q/ESC      Quit
```

## Development

```bash
# Run tests
cargo test

# Linting
cargo clippy

# Format
cargo fmt

# Documentation
cargo doc --open
```

## Roadmap

### Phase 0: Foundation (Complete)
- Rendering system
- Time control
- Zoom levels
- Game loop
- Input system
- World state

### Phase 1: Navigation & Maps (In Progress)
- Arrow key navigation
- Procedural galaxy generation
- Solar system maps
- Planet surface maps
- Camera and viewport system

### Phase 2: Economic Primitives (Planned)
- Resource types and distribution
- Basic agent AI
- Market mechanics
- Production and consumption
- Price formation

### Phase 3: Simulation Depth
- Complex agent behaviors
- Multi-commodity markets
- Trade routes and logistics
- Economic events
- Player interaction

## Design Philosophy

1. Authentic economic models over gamified abstractions
2. Multi-scale thinking - connections between micro and macro
3. Emergent complexity from simple rules
4. Interactive learning through participation
5. Transparent mechanisms

## License

MIT - see [LICENSE](LICENSE)
