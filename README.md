# Econogenesis

**A terminal-based real-time universe simulator exploring economics through interactive simulation**

Navigate from individual rooms to galactic scales, observing how economic forces shape agents, resources, and environments in a procedurally generated universe.

[![Rust](https://img.shields.io/badge/rust-1.80%2B-orange.svg)](https://www.rust-lang.org)
[![License](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE)
[![Status](https://img.shields.io/badge/status-in%20development-yellow.svg)]()

---

## 🎮 Overview

Econogenesis places you inside a living, evolving economic universe. It blends simulation and strategy to let you experience how systems emerge from simple rules.

**Core Concepts:**
- **Multi-scale navigation** – Move seamlessly from a single room to entire galaxies
- **Real-time simulation** – Control time flow with play, pause, and speed adjustments
- **Economic emergence** – Watch markets and behaviors arise naturally from scarcity and incentives
- **Interactive learning** – Understand economic ideas by participating directly in the simulation

---

## 📊 Current Status

### ✅ Foundation Complete (Phases 1-6)

The core infrastructure is **100% complete** and production-ready:

| Phase | Component | Status | Description |
|-------|-----------|--------|-------------|
| 1 | **Rendering System** | ✅ Complete | Terminal-based canvas with tty-interface, 30-60 FPS |
| 2 | **Time Control** | ✅ Complete | Play/pause, variable speed (0.1x to 50x) |
| 3 | **Zoom Levels** | ✅ Complete | 6-level hierarchy (Room → Galaxy) |
| 4 | **Game Loop** | ✅ Complete | Clean input → update → render architecture |
| 5 | **Input System** | ✅ Complete | Action-based keyboard controls + help overlay |
| 6 | **World State** | ✅ Complete | Multi-scale entity system with sample data |

**Statistics:**
- ✅ 18 unit tests passing
- ✅ Zero warnings, zero clippy violations
- ✅ ~1,500 lines of well-tested Rust code
- ✅ Clean, modular architecture

### 🚧 In Development: Navigation & Maps

**Current Sprint:** Building explorable, procedurally generated universe

**Target Features:**
- Arrow key navigation within each zoom level
- Enter/exit entities to zoom in/out
- Procedurally generated galaxy with 100-1000 star systems
- Unique planets, regions, and locations
- Camera system with viewport management
- Persistent, deterministic world generation

**Documentation:**
- 📋 [Navigation & Maps Plan](docs/navigation-and-maps-plan.md) - Detailed implementation plan
- 📋 [Foundation Plan](docs/foundation-plan.md) - Completed infrastructure

---

## 🎯 Features

### Currently Implemented

#### Multi-Scale Zoom System
Navigate through 6 levels of perspective:
- **Galaxy** – Thousands of star systems across spiral arms
- **Solar System** – Planets orbiting their star
- **Planet** – Continental surfaces with varied terrain
- **Region** – Named areas with distinct characteristics
- **Local Area** – Buildings and structures
- **Room** – Individual interiors with details

#### Time Control
- ⏯️ Play/Pause simulation
- ⚡ Speed control: 0.1x to 50x (8 preset speeds)
- 📊 Real-time FPS counter
- ⏱️ Simulation time tracking with formatted display

#### User Interface
- 🎨 Clean terminal-based rendering (tty-interface)
- 📱 Dynamic viewport with auto-clear
- ❓ Help overlay (H/?) with all controls
- 📍 Position and location tracking
- 🔄 Smooth 30-60 FPS performance

### Planned Features

#### Navigation System (Next Milestone)
- **Arrow key movement** within each zoom level
- **Enter key** to zoom into entities
- **Procedural generation** of explorable universe
- **Camera system** with viewport management
- **Map rendering** with ASCII/Unicode art
- **Entity information** on hover/selection

#### Economic Simulation (Future)
- **Resource systems** – Scarcity and distribution
- **Agent AI** – Autonomous decision-making
- **Markets** – Price discovery and trade
- **Production chains** – Manufacturing and consumption
- **Trade routes** – Interplanetary commerce
- **Emergence** – Complex behavior from simple rules

---

## 🚀 Getting Started

### Requirements

- **Rust:** 1.80+ (2024 edition)
- **Terminal:** Unicode-capable terminal emulator
- **OS:** Linux, macOS, or Windows (WSL recommended)

### Installation

```bash
# Clone the repository
git clone https://github.com/yourusername/econogenesis.git
cd econogenesis

# Build (debug)
cargo build

# Build (release - optimized)
cargo build --release
```

### Run

```bash
# Run debug build
cargo run

# Run release build (better performance)
cargo run --release
```

### Controls

```
SPACE      Play/Pause simulation
+/=        Increase time speed
-/_        Decrease time speed
Z          Zoom in (closer view)
X          Zoom out (wider view)
H/?        Toggle help overlay
Q/ESC      Quit application
```

---

## 🏗️ Architecture

### Project Structure

```
econogenesis/
├── src/
│   ├── main.rs              # Entry point (30 lines!)
│   ├── game/                # Game loop and world state
│   │   ├── mod.rs
│   │   ├── game_loop.rs     # Main game loop
│   │   └── state.rs         # World state management
│   ├── input/               # Input handling
│   │   ├── mod.rs
│   │   └── handler.rs       # Action-based input system
│   ├── render/              # Terminal rendering
│   │   ├── mod.rs
│   │   ├── engine.rs        # Render engine with FPS
│   │   └── canvas.rs        # Drawing primitives
│   ├── time/                # Time control
│   │   ├── mod.rs
│   │   └── controller.rs    # Pause, speed, simulation time
│   ├── zoom/                # Multi-scale zoom system
│   │   ├── mod.rs
│   │   └── manager.rs       # Zoom levels and position
│   └── result.rs            # Error handling
├── docs/                    # Documentation
│   ├── foundation-plan.md          # Core infrastructure (COMPLETE)
│   └── navigation-and-maps-plan.md # Navigation system (IN PROGRESS)
└── tests/                   # Integration tests (future)
```

### Technology Stack

| Component | Technology |
|-----------|-----------|
| Language | Rust 1.80+ (2024 edition) |
| UI Framework | [tty-interface](https://crates.io/crates/tty-interface) 4.0.2 |
| Input Handling | [crossterm](https://crates.io/crates/crossterm) 0.25 |
| Error Handling | [thiserror](https://crates.io/crates/thiserror) 2.0 |

---

## 🧪 Development

### Commands

```bash
# Run tests (18 tests, all passing)
cargo test

# Run with clippy (linting)
cargo clippy

# Format code
cargo fmt

# Build documentation
cargo doc --open
```

### Testing

```bash
# Run all tests
cargo test

# Run specific test
cargo test test_name

# Run with output
cargo test -- --nocapture
```

### Code Quality

- ✅ Zero clippy warnings
- ✅ Zero compiler warnings
- ✅ Comprehensive unit tests
- ✅ Clean, documented code
- ✅ Modular architecture

---

## 📚 Documentation

### User Guides
- [Foundation Plan](docs/foundation-plan.md) - Core infrastructure implementation
- [Navigation & Maps Plan](docs/navigation-and-maps-plan.md) - Upcoming navigation system

### Design Documents (Future)
- Architecture Overview
- Economic Model Design
- Agent AI Design
- Procedural Generation Algorithms

---

## 🎯 Roadmap

### ✅ Phase 0: Foundation (Complete)
- [x] Rendering system
- [x] Time control
- [x] Zoom levels
- [x] Game loop
- [x] Input system
- [x] World state

### 🚧 Phase 1: Navigation & Maps (In Progress)
- [ ] Arrow key navigation
- [ ] Procedural galaxy generation
- [ ] Solar system maps
- [ ] Planet surface maps
- [ ] Region and area maps
- [ ] Camera and viewport system

### 📋 Phase 2: Economic Primitives (Planned)
- [ ] Resource types and distribution
- [ ] Basic agent AI
- [ ] Simple market mechanics
- [ ] Production and consumption
- [ ] Price formation

### 📋 Phase 3: Simulation Depth (Planned)
- [ ] Complex agent behaviors
- [ ] Multi-commodity markets
- [ ] Trade routes and logistics
- [ ] Economic events and crises
- [ ] Player interaction and strategy

### 📋 Phase 4: Polish & Content (Future)
- [ ] Rich world generation
- [ ] Narrative elements
- [ ] Advanced visualizations
- [ ] Save/load system
- [ ] Performance optimization

---

## 🎓 Vision

Econogenesis makes economics **visible and experiential**. Players move through layers of complexity, observing how local choices ripple outward to shape planetary and galactic systems.

### Learning Goals

The simulation builds intuition for:
- **Supply and demand** in dynamic systems
- **Resource management** and scarcity
- **Market formation** and price discovery
- **Economic externalities** and feedback loops
- **Scale-dependent effects** (micro to macro)
- **Emergence** – complex behavior from simple rules

### Design Philosophy

1. **Simulation First** – Authentic economic models, not game-ified abstractions
2. **Multi-Scale Thinking** – Understand connections between scales
3. **Emergence Over Prescription** – Let complexity arise naturally
4. **Interactive Learning** – Learn by doing, not reading
5. **Transparency** – Show the mechanisms, not just outcomes

---

## 🤝 Contributing

Econogenesis is in active development. Contributions welcome!

### Areas of Interest
- Procedural generation algorithms
- Economic modeling and agent AI
- Terminal UI/UX improvements
- Performance optimization
- Documentation and examples

### Getting Involved
1. Check out the [Navigation & Maps Plan](docs/navigation-and-maps-plan.md)
2. Look for "good first issue" tasks (coming soon)
3. Read the architecture docs (coming soon)
4. Join discussions and propose ideas

---

## 📄 License

MIT License - see [LICENSE](LICENSE) file for details

---

## 🙏 Acknowledgments

- Built with [Rust](https://www.rust-lang.org/) and the amazing Rust ecosystem
- Terminal rendering via [tty-interface](https://crates.io/crates/tty-interface)
- Inspired by economic simulation games and complexity science

---

**Current Version:** 0.1.0 (Foundation Complete)
**Last Updated:** 2025-10-08
**Status:** 🚧 Active Development - Navigation System In Progress
