# Foundation Plan

Core simulator infrastructure implementation.

## Objectives

Minimal viable foundation:

1. Rendering system - terminal canvas with tty-interface
2. Time control - play/pause and speed adjustment
3. Zoom system - multi-scale view hierarchy
4. Simulation loop - frame-based update cycle
5. Input handling - keyboard controls

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

## Phase 2: Time Control System ✅ COMPLETED

### Goals
- Implement simulation time tracking
- Add play/pause functionality
- Support variable speed control
- Maintain consistent timestep for deterministic simulation

### Tasks
- [x] Create `TimeController` struct
- [x] Implement play/pause state management
- [x] Add speed multiplier (0.1x, 0.5x, 1x, 2x, 5x, 10x, etc.)
- [x] Track simulation time vs real time
- [x] Display time controls in UI
- [x] Add keyboard shortcuts (SPACE for play/pause, +/- for speed)

### Implementation Summary
Created a complete time control system for managing simulation time, play/pause state, and speed control:

**Architecture:**
- `TimeController` manages simulation time independently from real time
- Starts in paused state by default
- Supports speed multipliers from 0.1x to 50x with smooth transitions
- Non-blocking keyboard input using crossterm events
- Fixed timestep with configurable target FPS

**Key Design Decisions:**
1. **Separate sim time from real time**: Allows deterministic simulation at any speed
2. **Non-blocking input**: Uses `crossterm::event::poll` with zero timeout
3. **Speed levels**: Predefined steps (0.1x, 0.5x, 1x, 2x, 5x, 10x, 20x, 50x) for intuitive control
4. **Time formatting**: Human-readable display (days, hours, minutes, seconds)
5. **Frame timing reset**: Prevents time jumps when unpausing

**Files Created/Modified:**
- [src/time/mod.rs](../src/time/mod.rs) - Module exports
- [src/time/controller.rs](../src/time/controller.rs) - TimeController with full implementation and tests
- [src/main.rs](../src/main.rs) - Integrated time control into game loop with keyboard input

**Features Implemented:**
- Play/Pause toggle (SPACE key)
- Speed increase/decrease (+/- keys)
- Quit functionality (Q/ESC keys)
- Real-time UI updates showing pause state, speed, and simulation time
- Clock animation that advances with simulation time

**Testing:** All 7 unit tests pass, covering:
- Initial paused state
- Pause toggling
- Speed increase/decrease
- Time advancement when playing
- Time freeze when paused
- Time formatting

**Keyboard Controls:**
```
SPACE    - Play/Pause
+/=      - Increase speed
-/_      - Decrease speed
Q/ESC    - Quit
```

### Actual Implementation
```rust
pub struct TimeController {
    is_paused: bool,
    speed_multiplier: f64,
    simulation_time: Duration,
    last_update: Instant,
    target_fps: u32,
}

impl TimeController {
    pub fn new(target_fps: u32) -> Self;
    pub fn is_paused(&self) -> bool;
    pub fn toggle_pause(&mut self);
    pub fn speed_multiplier(&self) -> f64;
    pub fn increase_speed(&mut self);
    pub fn decrease_speed(&mut self);
    pub fn simulation_time(&self) -> Duration;
    pub fn delta_time(&self) -> Duration;
    pub fn step(&mut self) -> Duration;
    pub fn format_time(&self) -> String;
}
```

## Phase 3: Zoom Level System ✅ COMPLETED

### Goals
- Define scale hierarchy
- Implement zoom level transitions
- Create scale-appropriate rendering
- Handle position translation between scales

### Tasks
- [x] Define `ZoomLevel` enum (Room, LocalArea, Region, Planet, SolarSystem, Galaxy)
- [x] Create `ZoomManager` to track current level
- [x] Implement zoom in/out with keyboard (Z/X keys)
- [x] Add position context preservation across zoom changes
- [x] Create placeholder views for each zoom level
- [x] Display current zoom level in UI

### Implementation Summary
Created a complete multi-scale zoom system with six distinct levels from room to galaxy scale:

**Architecture:**
- `ZoomLevel` enum with ordered levels supporting zoom in/out transitions
- `ZoomManager` tracks current zoom level and player position
- Multi-scale `Position` struct with nested coordinates for precision
- Visual representations for each zoom level
- Keyboard controls integrated into main game loop

**Key Design Decisions:**
1. **Ordered enum**: `ZoomLevel` implements `Ord` for level comparisons
2. **Bounded transitions**: `zoom_in()/zoom_out()` return `Option` to indicate limits
3. **Scale metadata**: `scale_meters()` provides physical scale reference for each level
4. **Multi-scale coordinates**: Position maintains nested coordinate systems
5. **Visual variety**: Each zoom level has distinct ASCII art representation

**Files Created/Modified:**
- [src/zoom/mod.rs](../src/zoom/mod.rs) - Module exports
- [src/zoom/manager.rs](../src/zoom/manager.rs) - ZoomLevel, Position, and ZoomManager implementation
- [src/main.rs](../src/main.rs) - Integrated zoom controls and rendering

**Features Implemented:**
- Six zoom levels with smooth transitions
- Z/X keyboard controls for zoom in/out
- Visual feedback showing current level in header
- Unique placeholder view for each scale
- Position tracking with multi-scale coordinates
- Boundary detection (can't zoom beyond Room or Galaxy)

**Testing:** All 7 unit tests pass, covering:
- Zoom in/out transitions for all levels
- Boundary conditions (can't zoom beyond limits)
- ZoomManager initialization
- Level ordering verification

**Keyboard Controls:**
```
Z        - Zoom in (Galaxy → Solar System → Planet → Region → Local Area → Room)
X        - Zoom out (Room → Local Area → Region → Planet → Solar System → Galaxy)
```

**Visual Representations:**
- **Galaxy View**: Stars and nebulae
- **Solar System View**: Sun and planets in orbit
- **Planet View**: Oceans and mountains
- **Region View**: Trees and landscape features
- **Local Area View**: Buildings and structures
- **Room View**: Furniture and interior details

### Actual Implementation
```rust
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum ZoomLevel {
    Room,
    LocalArea,
    Region,
    Planet,
    SolarSystem,
    Galaxy,
}

impl ZoomLevel {
    pub fn zoom_in(self) -> Option<Self>;
    pub fn zoom_out(self) -> Option<Self>;
    pub fn scale_meters(self) -> f64;
}

#[derive(Debug, Clone, Copy)]
pub struct Position {
    pub galaxy_coords: (f64, f64, f64),
    pub system_coords: (f64, f64, f64),
    pub planet_coords: (f64, f64, f64),
    pub local_coords: (f64, f64, f64),
}

pub struct ZoomManager {
    current_level: ZoomLevel,
    position: Position,
}

impl ZoomManager {
    pub fn new() -> Self;
    pub fn current_level(&self) -> ZoomLevel;
    pub fn position(&self) -> &Position;
    pub fn zoom_in(&mut self) -> bool;
    pub fn zoom_out(&mut self) -> bool;
}
```

## Phase 4: Simulation Loop Integration ✅ COMPLETED

### Goals
- Integrate all components into a unified game loop
- Establish frame timing and update cadence
- Create clean separation between update and render phases

### Tasks
- [x] Create main `GameLoop` struct
- [x] Implement fixed timestep with variable rendering
- [x] Add update() and render() phases
- [x] Integrate input handling
- [x] Add graceful shutdown handling
- [x] Implement basic state management

### Implementation Summary
Refactored the entire application into a clean, modular architecture with proper separation of concerns:

**Architecture:**
- `GameLoop` struct owns all game systems and manages the main loop
- `InputHandler` encapsulates all keyboard input with action-based API
- `WorldState` placeholder for future economic simulation state
- Main.rs reduced to minimal initialization code
- Clean separation: input → update → render → sleep cycle

**Key Design Decisions:**
1. **Ownership model**: GameLoop owns all subsystems and consumes self on run()
2. **Action-based input**: InputHandler returns InputAction enums instead of directly mutating state
3. **Static rendering**: draw_game() is a static method to avoid borrowing issues
4. **RenderState struct**: Groups rendering parameters to avoid too-many-arguments clippy warning
5. **Help overlay**: Toggle with H/? key to show all controls

**Files Created/Modified:**
- [src/game/mod.rs](../src/game/mod.rs) - Game module exports
- [src/game/game_loop.rs](../src/game/game_loop.rs) - Complete GameLoop implementation with all rendering logic
- [src/game/state.rs](../src/game/state.rs) - WorldState placeholder with tick counter
- [src/input/mod.rs](../src/input/mod.rs) - Input module exports
- [src/input/handler.rs](../src/input/handler.rs) - InputHandler with action-based API
- [src/main.rs](../src/main.rs) - Minimal initialization (30 lines vs 235 lines)
- [src/zoom/mod.rs](../src/zoom/mod.rs) - Added Position export

**Features Implemented:**
- Clean game loop with proper phase separation
- Input handling via InputAction enum
- Help overlay showing all keyboard controls (H/? to toggle)
- World state tracking with tick counter
- Graceful shutdown with terminal cleanup
- All previous features maintained (time control, zoom, FPS display)

**Testing:** All 13 unit tests pass, zero clippy warnings, clean build

**Updated Keyboard Controls:**
```
SPACE    - Play/Pause
+/=      - Increase speed
-/_      - Decrease speed
Z        - Zoom in
X        - Zoom out
H/?      - Toggle help overlay
Q/ESC    - Quit
```

### Actual Implementation
```rust
pub struct GameLoop<'a> {
    render_engine: RenderEngine<'a>,
    time_controller: TimeController,
    zoom_manager: ZoomManager,
    world_state: WorldState,
    input_handler: InputHandler,
}

impl GameLoop {
    pub fn run(mut self) -> Result<()> {
        loop {
            if self.handle_input()? { break; }
            if !self.time_controller.is_paused() {
                self.update();
            }
            self.render()?;
            sleep(self.time_controller.target_frame_duration());
        }
        self.render_engine.exit()?;
        Ok(())
    }

    fn handle_input(&mut self) -> Result<bool>;
    fn update(&mut self);
    fn render(&mut self) -> Result<()>;
}

pub enum InputAction {
    Quit, TogglePause, IncreaseSpeed, DecreaseSpeed,
    ZoomIn, ZoomOut, ToggleHelp, None,
}

pub struct InputHandler {
    show_help: bool,
}

impl InputHandler {
    pub fn poll(&mut self) -> Result<InputAction>;
    pub fn is_help_visible(&self) -> bool;
}

pub struct WorldState {
    tick_count: u64,
}

impl WorldState {
    pub fn update(&mut self, delta: Duration);
    pub fn tick_count(&self) -> u64;
}
```

## Phase 5: Input System ✅ COMPLETED (as part of Phase 4)

### Goals
- Handle keyboard input non-blocking
- Map keys to actions
- Support modifier keys
- Provide visual feedback

### Tasks
- [x] Create `InputHandler` for keyboard events
- [x] Define key mappings (play/pause, zoom, speed, quit)
- [x] Implement non-blocking input polling
- [x] Create help overlay showing controls
- [ ] Add command buffer for complex inputs (future feature)
- [ ] Support arrow keys for navigation (future feature)

### Implementation Summary
Input system was implemented as part of Phase 4 refactoring.

**Key Features:**
- Action-based input design with `InputAction` enum
- Non-blocking polling using crossterm with zero timeout
- Help overlay toggle integrated into InputHandler
- All basic controls implemented and working

**Completed in Phase 4:** See Phase 4 implementation for full details.

### Current Key Bindings
```
SPACE    - Play/Pause
+/=      - Increase speed
-/_      - Decrease speed
Z        - Zoom in
X        - Zoom out
H/?      - Toggle help overlay
Q/ESC    - Quit
```

**Future Extensions:**
- Arrow keys for player movement
- Command buffer for complex multi-key inputs
- Configurable key bindings

## Phase 6: Basic World State ✅ COMPLETED

### Goals
- Create minimal world representation
- Support multi-scale state storage
- Enable position tracking

### Tasks
- [x] Define `WorldState` struct
- [x] Implement basic update mechanism
- [x] Create placeholder data at each zoom level
- [x] Implement player position tracking
- [x] Create simple test data for each scale
- [ ] Add basic serialization for save/load (future feature)

### Implementation Summary
Created a complete multi-scale world state system with placeholder entities at every zoom level:

**Architecture:**
- `WorldState` manages all world entities across 6 scales
- Separate state structs for each zoom level (Galaxy, Solar System, Planet, Region, Local Area, Room)
- HashMap-based storage with EntityId keys
- Sample data initialized for testing and demonstration
- Player position tracking with multi-scale coordinates

**Key Design Decisions:**
1. **Type-safe entity IDs**: `EntityId` type alias for u64
2. **Scale-specific structures**: Each zoom level has its own state struct with relevant fields
3. **HashMap storage**: Efficient O(1) lookup for entities by ID
4. **Sample data initialization**: Each scale has at least one example entity
5. **Context-aware display**: `get_current_entity_name()` returns appropriate entity name for current zoom

**Files Created/Modified:**
- [src/game/state.rs](../src/game/state.rs) - Complete WorldState implementation with 6 scale-specific structures
- [src/game/game_loop.rs](../src/game/game_loop.rs) - Updated to display current location name and entity count

**Features Implemented:**
- Multi-scale world representation (Galaxy → Room)
- Player position tracking
- Sample entities at each scale:
  - Galaxy: "Andromeda Prime" (1B stars)
  - Solar System: "Sol System" (8 planets)
  - Planet: "Terra" (7.8B population)
  - Region: "Northern Highlands" (Mountains)
  - Local Area: "Market District" (47 buildings)
  - Room: "Trading Hall" (Commercial)
- Entity count tracking
- Context-aware UI display

**Testing:** All 18 unit tests pass (5 new WorldState tests), covering:
- World state initialization
- Update mechanism
- Sample data existence
- Entity name retrieval for all zoom levels
- Player position tracking

### Actual Implementation
```rust
pub type EntityId = u64;

#[derive(Debug, Clone)]
pub struct GalaxyState {
    pub name: String,
    pub star_count: u64,
}

#[derive(Debug, Clone)]
pub struct SolarSystemState {
    pub id: EntityId,
    pub name: String,
    pub planet_count: u32,
}

// Similar structs for: PlanetState, RegionState, LocalAreaState, RoomState

pub struct WorldState {
    tick_count: u64,
    player_position: Position,
    galaxy: GalaxyState,
    systems: HashMap<EntityId, SolarSystemState>,
    planets: HashMap<EntityId, PlanetState>,
    regions: HashMap<EntityId, RegionState>,
    areas: HashMap<EntityId, LocalAreaState>,
    rooms: HashMap<EntityId, RoomState>,
}

impl WorldState {
    pub fn new() -> Self;
    pub fn update(&mut self, delta: Duration);
    pub fn get_current_entity_name(&self, zoom_level: ZoomLevel) -> String;
    pub fn entity_count(&self) -> usize;
    // Getters for each entity type...
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

## Status: Complete

All 6 foundation phases complete:

- Phase 1: Rendering System
- Phase 2: Time Control System
- Phase 3: Zoom Level System
- Phase 4: Simulation Loop Integration
- Phase 5: Input System
- Phase 6: Basic World State

**Current state:**
- 18 unit tests passing, zero warnings
- Clean modular architecture
- Multi-scale world representation
- Full input system with help overlay

**Next:**
1. World generation - procedural galaxy/systems/planets/regions
2. Economic primitives - resources, goods, prices
3. Agents and markets
4. Player interaction

## Technical Considerations

### Performance
- Target 30-60 FPS
- Efficient terminal redrawing
- Double buffering to reduce flicker

### Testing
- Unit tests for core logic
- Integration tests for zoom transitions
- Manual testing across terminal emulators

### Platform Support
- Linux, macOS, Windows (via WSL)
- Handle various terminal sizes
