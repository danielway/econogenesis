# Foundation Plan: Core Simulator Infrastructure

This document outlines the plan for building the foundational components of Econogenesis. The goal is to establish the basic architecture that supports real-time simulation, multi-scale rendering, and time control.

## Objectives

Build the minimal viable foundation that includes:

1. **Rendering System**: Terminal-based canvas using tty-interface
2. **Time Control**: Play/pause and speed adjustment
3. **Zoom System**: Multi-scale view hierarchy and navigation
4. **Simulation Loop**: Frame-based update cycle
5. **Input Handling**: Keyboard controls for time and zoom

## Phase 1: Basic Rendering Framework ✅ COMPLETED

### Goals
- Initialize tty-interface
- Create a basic canvas that can display text and simple graphics
- Implement a render loop

### Tasks
- [x] Set up tty-interface with proper terminal initialization/cleanup
- [x] Create `RenderEngine` struct to manage terminal state
- [x] Implement basic canvas abstraction for drawing
- [x] Add FPS counter for development
- [x] Handle terminal resize events

### Implementation Summary
Created the rendering infrastructure with proper separation of concerns:

**Architecture:**
- `Canvas` owns the `Interface` and handles all terminal drawing operations
- `RenderEngine` wraps `Canvas` and manages frame timing/FPS tracking
- Custom `Result` type unifies error handling across tty-interface and std::io
- Lifetime-based design (`'a`) ensures proper device ownership without `Box::leak`

**Key Design Decisions:**
1. **Canvas owns Interface**: Simplified borrowing by consolidating drawing operations
2. **Device trait abstraction**: Uses `&'a mut dyn Device` instead of concrete `Stdout`, enabling testing and flexibility
3. **Error type unification**: Created `src/result.rs` with custom `Error` enum using thiserror
4. **Mutable canvas API**: Drawing methods take `&mut self`, avoiding complex borrow splitting
5. **Proper separation**: `handle_frame()` and `draw_game()` separate engine logic from drawing

**Files Created:**
- [src/result.rs](../src/result.rs) - Custom Result/Error types
- [src/render/mod.rs](../src/render/mod.rs) - Module exports
- [src/render/engine.rs](../src/render/engine.rs) - RenderEngine with FPS tracking
- [src/render/canvas.rs](../src/render/canvas.rs) - Canvas with Interface ownership
- [src/main.rs](../src/main.rs) - Clean separation: run() → handle_frame() → draw_game()

**Dependencies:**
- tty-interface 4.0.2 (updated from 4.0.1)
- crossterm 0.25
- thiserror 2.0.16 (new)

**Testing:** The application requires an interactive terminal with a Device implementation (typically stdout).

### Actual Implementation
```rust
// Canvas owns the Interface
pub struct Canvas<'a> {
    interface: Interface<'a>,
    width: u16,
    height: u16,
}

impl<'a> Canvas<'a> {
    pub fn new(device: &'a mut dyn Device) -> Result<Canvas<'a>>;
    pub fn draw_text(&mut self, x: u16, y: u16, text: &str);
    pub fn draw_horizontal_line(&mut self, x: u16, y: u16, length: u16, ch: char);
    pub fn draw_box(&mut self, x: u16, y: u16, width: u16, height: u16);
    pub fn apply_staged_updates(&mut self) -> Result<()>;
}

// RenderEngine wraps Canvas and adds FPS tracking
pub struct RenderEngine<'a> {
    canvas: Canvas<'a>,
    frame_count: u64,
    last_fps_update: Instant,
    current_fps: f32,
    frames_since_last_update: u32,
}

// Custom error handling
pub type Result<T> = std::result::Result<T, Error>;

#[derive(ThisError, Debug)]
pub enum Error {
    #[error("terminal IO error")]
    TerminalError(#[from] std::io::Error),
    #[error("terminal interface error")]
    InterfaceError(#[from] tty_interface::Error),
}
```

## Phase 2: Time Control System

### Goals
- Implement simulation time tracking
- Add play/pause functionality
- Support variable speed control
- Maintain consistent timestep for deterministic simulation

### Tasks
- [ ] Create `TimeController` struct
- [ ] Implement play/pause state management
- [ ] Add speed multiplier (0.1x, 0.5x, 1x, 2x, 5x, 10x, etc.)
- [ ] Track simulation time vs real time
- [ ] Display time controls in UI
- [ ] Add keyboard shortcuts (SPACE for play/pause, +/- for speed)

### Implementation Notes
```rust
struct TimeController {
    is_paused: bool,
    speed_multiplier: f64,
    simulation_time: Duration,
    last_update: Instant,
    target_fps: u32,
}

impl TimeController {
    fn delta_time(&self) -> Duration { /* ... */ }
    fn step(&mut self) { /* ... */ }
}
```

## Phase 3: Zoom Level System

### Goals
- Define scale hierarchy
- Implement zoom level transitions
- Create scale-appropriate rendering
- Handle position translation between scales

### Tasks
- [ ] Define `ZoomLevel` enum (Room, LocalArea, Region, Planet, SolarSystem, Galaxy)
- [ ] Create `ZoomManager` to track current level
- [ ] Implement zoom in/out with keyboard (Z/X keys or mouse wheel)
- [ ] Add position context preservation across zoom changes
- [ ] Create placeholder views for each zoom level
- [ ] Display current zoom level in UI

### Implementation Notes
```rust
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum ZoomLevel {
    Room,        // ~10m scale
    LocalArea,   // ~1km scale
    Region,      // ~100km scale
    Planet,      // ~10,000km scale
    SolarSystem, // ~10 AU scale
    Galaxy,      // ~100,000 ly scale
}

struct ZoomManager {
    current_level: ZoomLevel,
    position: Position,  // Player position in world coordinates
}

struct Position {
    galaxy_coords: (f64, f64, f64),
    // Additional nested coordinates for precision at smaller scales
    system_coords: (f64, f64, f64),
    planet_coords: (f64, f64, f64),
    local_coords: (f64, f64, f64),
}
```

## Phase 4: Simulation Loop Integration

### Goals
- Integrate all components into a unified game loop
- Establish frame timing and update cadence
- Create clean separation between update and render phases

### Tasks
- [ ] Create main `GameLoop` struct
- [ ] Implement fixed timestep with variable rendering
- [ ] Add update() and render() phases
- [ ] Integrate input handling
- [ ] Add graceful shutdown handling
- [ ] Implement basic state management

### Implementation Notes
```rust
struct GameLoop {
    render_engine: RenderEngine,
    time_controller: TimeController,
    zoom_manager: ZoomManager,
    world_state: WorldState,
    input_handler: InputHandler,
}

impl GameLoop {
    fn run(&mut self) {
        loop {
            self.handle_input();

            if !self.time_controller.is_paused {
                self.update();
            }

            self.render();
            self.sleep_until_next_frame();
        }
    }
}
```

## Phase 5: Input System

### Goals
- Handle keyboard input non-blocking
- Map keys to actions
- Support modifier keys
- Provide visual feedback

### Tasks
- [ ] Create `InputHandler` for keyboard events
- [ ] Define key mappings (play/pause, zoom, speed, quit)
- [ ] Implement non-blocking input polling
- [ ] Add command buffer for complex inputs
- [ ] Create help overlay showing controls

### Key Bindings (Proposed)
```
SPACE    - Play/Pause
+/=      - Increase speed
-/_      - Decrease speed
Z        - Zoom in
X        - Zoom out
Arrow    - Navigate/move
Q        - Quit
H/?      - Help overlay
```

## Phase 6: Basic World State

### Goals
- Create minimal world representation
- Support multi-scale state storage
- Enable position tracking

### Tasks
- [ ] Define `WorldState` struct
- [ ] Create placeholder data at each zoom level
- [ ] Implement player position tracking
- [ ] Add basic serialization for save/load (future)
- [ ] Create simple test data for each scale

### Implementation Notes
```rust
struct WorldState {
    player_position: Position,
    galaxy: GalaxyState,
    systems: HashMap<SystemId, SolarSystemState>,
    planets: HashMap<PlanetId, PlanetState>,
    regions: HashMap<RegionId, RegionState>,
    areas: HashMap<AreaId, LocalAreaState>,
    rooms: HashMap<RoomId, RoomState>,
}
```

## UI Layout (Initial)

```
┌─────────────────────────────────────────────────────────────┐
│ Econogenesis v0.1.0          Galaxy View     [PAUSED] 1.0x │
├─────────────────────────────────────────────────────────────┤
│                                                              │
│                                                              │
│                    [Main Canvas Area]                        │
│                                                              │
│                         *  ·                                 │
│                    ·         Player                          │
│                        ·   *                                 │
│                                                              │
│                                                              │
├─────────────────────────────────────────────────────────────┤
│ Time: 1d 3h 42m | Pos: (1234, 5678) | FPS: 60              │
│ [SPACE] Play/Pause [Z/X] Zoom [+/-] Speed [Q] Quit         │
└─────────────────────────────────────────────────────────────┘
```

## Success Criteria

The foundation is complete when:

1. ✅ The application runs in a terminal with proper initialization/cleanup
2. ✅ Time can be paused/unpaused and speed adjusted
3. ✅ User can zoom in/out through all 6 levels
4. ✅ Each zoom level displays placeholder content
5. ✅ Player position is tracked and displayed
6. ✅ Simulation time is tracked and displayed
7. ✅ Keyboard controls work responsively
8. ✅ Application runs at stable framerate (30-60 fps)
9. ✅ Code is modular and well-structured for future expansion

## Non-Goals (For This Phase)

- Economic simulation logic
- Agent AI or decision-making
- Resource systems
- Complex graphics or animations
- Networking or multiplayer
- Save/load functionality
- Sound or music

## Next Steps After Foundation

Once the foundation is solid, development can proceed to:

1. **World Generation**: Procedural generation of galaxy → systems → planets → regions
2. **Economic Primitives**: Resources, agents, simple markets
3. **Agent Behavior**: Basic decision-making and movement
4. **UI Enhancement**: Better visualizations, menus, info panels
5. **Game Mechanics**: Player actions, inventory, transactions

## Estimated Timeline

- **Phase 1**: 2-3 days (Rendering)
- **Phase 2**: 1-2 days (Time Control)
- **Phase 3**: 2-3 days (Zoom System)
- **Phase 4**: 1-2 days (Integration)
- **Phase 5**: 1-2 days (Input)
- **Phase 6**: 1-2 days (World State)

**Total**: ~2 weeks for a solid foundation

## Technical Considerations

### Performance
- Target 30-60 FPS in terminal
- Efficient terminal redrawing (only update changed cells)
- Consider double buffering to reduce flicker

### Testing
- Unit tests for time control logic
- Integration tests for zoom transitions
- Manual testing in different terminal emulators

### Platform Support
- Test on Linux, macOS, Windows (via WSL or native)
- Ensure compatibility with common terminal emulators
- Handle various terminal sizes gracefully

### Code Quality
- Follow Rust idioms and best practices
- Document public APIs
- Keep modules loosely coupled
- Use traits for extensibility
