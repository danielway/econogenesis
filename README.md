# Econogenesis

A terminal-based real-time universe simulator that explores economics through interactive simulation. Players can navigate from individual rooms to galactic scales, observing how economic forces shape agents, resources, and environments.

## Overview

Econogenesis places you inside a living, evolving economic universe. It blends simulation and strategy to let you experience how systems emerge from simple rules. Core concepts include:

- **Multi-scale navigation** – Move seamlessly from a single room to entire galaxies
- **Real-time simulation** – Control time flow with play, pause, and speed adjustments
- **Economic emergence** – Watch markets and behaviors arise naturally from scarcity and incentives
- **Interactive learning** – Understand economic ideas by participating directly in the simulation

## Features

### Development Status

- 🚧 **In Development** – Core foundation in progress

### Planned Features

- **Scalable Worlds**

  - **Room** – Local interactions between agents and resources
  - **Local Area** – Neighborhood and community-level systems
  - **Region** – Towns, cities, and larger territories
  - **Planet** – Continental and global networks
  - **Solar System** – Planetary trade and resource flow
  - **Galaxy** – Civilization-scale economics and interstellar trade

- **Time Control**

  - Play/pause the simulation
  - Adjust simulation speed
  - Maintain real-time state consistency across all scales

- **Economic Simulation**

  - Resource scarcity and environmental limits
  - Agent autonomy and adaptive decision-making
  - Market formation and price discovery
  - Production, consumption, and trade dynamics

- **User Experience**

  - Third-person agent perspective
  - Context-aware UI and controls per scale
  - Strategic decision-making and visualization tools

## Technology

- **Language:** Rust
- **UI Framework:** [tty-interface](https://crates.io/crates/tty-interface)
- **Platform:** Cross-platform terminal

## Getting Started

### Requirements

- Rust 1.80+ (edition 2024)
- Terminal emulator with Unicode support

### Installation

```bash
git clone https://github.com/yourusername/econogenesis.git
cd econogenesis
cargo build --release
```

### Run the Game

```bash
cargo run --release
```

## Development

### Structure

```
econogenesis/
├── src/
│   ├── main.rs           # Entry point
│   ├── simulation/       # Core simulation logic
│   ├── render/           # Terminal UI rendering
│   ├── economics/        # Agent and market systems
│   └── world/            # Spatial hierarchy and world state
├── docs/                 # Documentation
└── tests/                # Integration tests
```

### Commands

```bash
# Development build
cargo build

# Release build
cargo build --release

# Run tests
cargo test
```

## Vision

Econogenesis makes economics visible and experiential. Players move through layers of complexity, observing how local choices ripple outward to shape planetary and galactic systems. The simulation aims to build intuition for:

- Supply and demand in dynamic systems
- Resource management and scarcity
- Market formation and disruption
- Economic externalities and feedback loops
- Scale-dependent effects
- Complex behavior emerging from simple rules
