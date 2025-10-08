# Multi-Scale Navigation & Map System Plan

**Status:** 📋 Planning Phase
**Target:** Make Econogenesis a fully explorable, procedurally generated universe
**Scope:** Complete navigation system across all 6 zoom levels with procedural map generation

---

## Overview

Transform Econogenesis from a static demonstration into an explorable universe with procedurally generated content and full navigation across all 6 zoom levels:

**Galaxy** → **Solar System** → **Planet** → **Region** → **Local Area** → **Room**

The player will be able to:
- Navigate within each zoom level using arrow keys
- Zoom into entities (systems, planets, regions, areas, rooms)
- Zoom out to parent levels while maintaining context
- Explore procedurally generated, persistent worlds
- See their position and available navigation options

---

## Phase 1: Navigation Infrastructure (Foundation)

### 1.1 Enhanced Position System

**Goal:** Enable player to navigate within each zoom level with proper tracking.

**File:** `src/zoom/manager.rs`

**Current Limitation:**
- Position only stores floating-point coordinates
- No concept of which specific entity the player is viewing
- No grid-based navigation within levels

**Proposed Changes:**

```rust
use crate::game::state::EntityId;

pub struct Position {
    // Entity ID tracking - which specific entity at each level
    pub current_system_id: Option<EntityId>,
    pub current_planet_id: Option<EntityId>,
    pub current_region_id: Option<EntityId>,
    pub current_area_id: Option<EntityId>,
    pub current_room_id: Option<EntityId>,

    // Grid coordinates for spatial navigation (integer-based)
    pub galaxy_coords: (i32, i32),      // Grid position in galaxy map
    pub system_coords: (i32, i32),      // Position within solar system
    pub planet_coords: (i32, i32),      // Position on planet surface
    pub region_coords: (i32, i32),      // Position within region
    pub area_coords: (i32, i32),        // Position in local area
    pub room_coords: (i32, i32),        // Position within room
}
```

**New Methods:**
```rust
impl Position {
    pub fn coords_for_level(&self, level: ZoomLevel) -> (i32, i32);
    pub fn set_coords_for_level(&mut self, level: ZoomLevel, coords: (i32, i32));
    pub fn current_entity_id(&self, level: ZoomLevel) -> Option<EntityId>;
}
```

### 1.2 Direction Enum

**File:** `src/zoom/manager.rs` or new `src/navigation/mod.rs`

```rust
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    pub fn to_offset(&self) -> (i32, i32) {
        match self {
            Direction::Up => (0, -1),
            Direction::Down => (0, 1),
            Direction::Left => (-1, 0),
            Direction::Right => (1, 0),
        }
    }
}
```

### 1.3 ZoomManager Navigation

**New Methods:**
```rust
impl ZoomManager {
    // Movement within current zoom level
    pub fn move_in_direction(&mut self, direction: Direction) -> bool;

    // Check if player can enter the entity at current position
    pub fn can_enter_current(&self, world_state: &WorldState) -> bool;

    // Enter the entity at current position (zoom in)
    pub fn enter_current(&mut self, world_state: &WorldState) -> bool;

    // Exit to parent level (zoom out to previous location)
    pub fn exit_to_parent(&mut self) -> bool;
}
```

### 1.4 Input Actions for Navigation

**File:** `src/input/handler.rs`

**New InputAction Variants:**
```rust
pub enum InputAction {
    // Existing actions...
    Quit,
    TogglePause,
    IncreaseSpeed,
    DecreaseSpeed,
    ZoomIn,      // Keep for backward compatibility
    ZoomOut,     // Keep for backward compatibility
    ToggleHelp,

    // New navigation actions
    MoveUp,      // Arrow Up
    MoveDown,    // Arrow Down
    MoveLeft,    // Arrow Left
    MoveRight,   // Arrow Right
    Enter,       // Enter key - zoom into current entity
    None,
}
```

**Key Mappings:**
```rust
KeyCode::Up => InputAction::MoveUp,
KeyCode::Down => InputAction::MoveDown,
KeyCode::Left => InputAction::MoveLeft,
KeyCode::Right => InputAction::MoveRight,
KeyCode::Enter => InputAction::Enter,
```

---

## Phase 2: Map Data Structures

### 2.1 Map Module Organization

**New Directory:** `src/map/`

**Files:**
- `src/map/mod.rs` - Module exports and common types
- `src/map/traits.rs` - Common traits for all map types
- `src/map/galaxy.rs` - Galaxy map structure and generation
- `src/map/system.rs` - Solar system layouts
- `src/map/planet.rs` - Planet surface maps
- `src/map/region.rs` - Regional terrain
- `src/map/area.rs` - Local area buildings/structures
- `src/map/room.rs` - Interior room layouts
- `src/map/generator.rs` - Procedural generation utilities (Perlin noise, etc.)

### 2.2 Common Map Trait

**File:** `src/map/traits.rs`

```rust
pub trait MapLevel {
    type Entity;

    fn width(&self) -> i32;
    fn height(&self) -> i32;
    fn get_entity_at(&self, x: i32, y: i32) -> Option<&Self::Entity>;
    fn can_navigate_to(&self, x: i32, y: i32) -> bool;
    fn get_visible_entities(&self, center: (i32, i32), radius: i32) -> Vec<&Self::Entity>;
    fn is_valid_position(&self, x: i32, y: i32) -> bool {
        x >= 0 && x < self.width() && y >= 0 && y < self.height()
    }
}
```

### 2.3 Galaxy Map

**File:** `src/map/galaxy.rs`

```rust
use crate::game::state::EntityId;
use std::collections::HashMap;

pub struct GalaxyMap {
    width: i32,
    height: i32,
    systems: HashMap<(i32, i32), EntityId>,
    system_data: HashMap<EntityId, SolarSystemData>,
}

pub struct SolarSystemData {
    id: EntityId,
    name: String,
    position: (i32, i32),
    star_type: StarType,
    planet_count: u32,
}

pub enum StarType {
    RedDwarf,
    YellowStar,
    BlueGiant,
    WhiteDwarf,
    NeutronStar,
}

impl GalaxyMap {
    pub fn generate(width: i32, height: i32, seed: u64) -> Self;
    pub fn get_system_at(&self, pos: (i32, i32)) -> Option<&SolarSystemData>;
}
```

### 2.4 Solar System Map

**File:** `src/map/system.rs`

```rust
pub struct SolarSystemMap {
    id: EntityId,
    width: i32,
    height: i32,
    star_position: (i32, i32),
    star_type: StarType,
    planets: HashMap<(i32, i32), EntityId>,
    planet_data: HashMap<EntityId, PlanetData>,
}

pub struct PlanetData {
    id: EntityId,
    name: String,
    position: (i32, i32),
    planet_type: PlanetType,
    has_atmosphere: bool,
    orbital_distance: f32,
}

pub enum PlanetType {
    Rocky,
    GasGiant,
    IceGiant,
    Desert,
    Ocean,
    Volcanic,
}

impl SolarSystemMap {
    pub fn generate(system_id: EntityId, star_type: StarType, seed: u64) -> Self;
}
```

### 2.5 Planet Map

**File:** `src/map/planet.rs`

```rust
pub struct PlanetMap {
    id: EntityId,
    width: i32,
    height: i32,
    terrain: Vec<Vec<TerrainType>>,
    regions: HashMap<(i32, i32), EntityId>,
    region_data: HashMap<EntityId, RegionData>,
}

pub enum TerrainType {
    Ocean,
    Plains,
    Mountains,
    Desert,
    Forest,
    Ice,
    Volcanic,
    Jungle,
}

pub struct RegionData {
    id: EntityId,
    name: String,
    center: (i32, i32),
    radius: i32,
    terrain_type: TerrainType,
}

impl PlanetMap {
    pub fn generate(planet_id: EntityId, planet_type: PlanetType, seed: u64) -> Self;
}
```

### 2.6 Region, Area, and Room Maps

**Similar hierarchical structure continues:**

- **Region Map** contains local areas (towns, cities, wilderness)
- **Local Area Map** contains buildings and structures
- **Room Map** contains furniture, objects, and NPCs

---

## Phase 3: Procedural Generation

### 3.1 Generation Utilities

**File:** `src/map/generator.rs`

**Core Utilities:**
- Perlin noise for terrain generation
- Seeded random number generation (deterministic)
- Name generation (for systems, planets, regions)
- Distribution algorithms (spiral arms for galaxies, orbits for systems)

```rust
pub struct NoiseGenerator {
    seed: u64,
}

impl NoiseGenerator {
    pub fn new(seed: u64) -> Self;
    pub fn noise_2d(&self, x: f64, y: f64, scale: f64) -> f64;
    pub fn octave_noise_2d(&self, x: f64, y: f64, octaves: u32, persistence: f64) -> f64;
}

pub struct NameGenerator {
    seed: u64,
}

impl NameGenerator {
    pub fn generate_star_name(&mut self) -> String;
    pub fn generate_planet_name(&mut self) -> String;
    pub fn generate_region_name(&mut self) -> String;
}
```

### 3.2 Galaxy Generation Algorithm

**Strategy:** Spiral arm distribution with density variations

1. Define 2-4 spiral arms
2. Use parametric equations for spiral curves
3. Add Perlin noise for variation
4. Place systems along spirals with random offsets
5. Vary density (more systems near galactic center)
6. Generate 100-1000 systems total

### 3.3 Solar System Generation

**Strategy:** Realistic orbital mechanics

1. Determine planet count based on star type:
   - Red dwarf: 0-4 planets
   - Yellow star: 2-12 planets
   - Blue giant: 0-3 planets
2. Place planets in orbital shells
3. Inner planets: rocky/desert
4. Outer planets: gas giants/ice
5. Add moons to some planets (future)

### 3.4 Planet Generation

**Strategy:** Terrain using 2D Perlin noise

1. Generate height map using Perlin noise
2. Assign terrain based on height thresholds:
   - < 0.3: Ocean
   - 0.3-0.4: Plains
   - 0.4-0.6: Forest/Desert (based on latitude)
   - 0.6-0.8: Mountains
   - > 0.8: Snow/Ice
3. Place 20-50 regions as named areas
4. Ensure regions don't overlap

### 3.5 Region/Area/Room Generation

**Progressive detail as player zooms in:**

- **Regions:** Settlements, roads, terrain features
- **Areas:** Individual buildings, NPCs, resources
- **Rooms:** Furniture, items, interactive objects

---

## Phase 4: Rendering System Updates

### 4.1 Camera System

**File:** `src/render/camera.rs`

```rust
pub struct Camera {
    viewport_width: u16,
    viewport_height: u16,
    center_x: i32,
    center_y: i32,
}

impl Camera {
    pub fn new(viewport_width: u16, viewport_height: u16) -> Self;

    pub fn set_center(&mut self, x: i32, y: i32);

    pub fn world_to_screen(&self, world_x: i32, world_y: i32) -> Option<(u16, u16)>;

    pub fn get_visible_bounds(&self) -> (i32, i32, i32, i32);
}
```

### 4.2 Map Renderer

**File:** `src/render/map_renderer.rs`

**Responsibilities:**
- Render visible portion of current map
- Convert world coordinates to screen coordinates
- Draw entities with appropriate symbols
- Highlight player position
- Show entity information

**Symbol Mappings:**

**Galaxy View:**
- `*` = Star system
- `·` = Empty space
- `@` = Player

**Solar System View:**
- `☉` = Star (different colors for types)
- `o` = Small planet
- `O` = Large planet
- `@` = Player ship

**Planet View:**
- `~` = Ocean
- `^` = Mountains
- `"` = Plains
- `♣` = Forest
- `#` = Desert
- `*` = Ice
- `@` = Player

**Region View:**
- ASCII art for settlements
- Terrain symbols
- Road connections

**Area/Room Views:**
- `▓` = Buildings/walls
- `░` = Floors
- `├─┤` = Doors
- `@` = Player

### 4.3 Dynamic View Rendering

**Replace static ASCII art in `game_loop.rs`:**

```rust
fn draw_zoom_view(canvas: &mut Canvas, state: &RenderState, world: &WorldState) {
    let camera = Camera::new(canvas.width() - 4, canvas.height() - 10);
    let map_renderer = MapRenderer::new(&camera);

    match state.zoom_level {
        ZoomLevel::Galaxy => {
            let map = world.galaxy_map();
            map_renderer.render_galaxy(canvas, map, state.position);
        }
        ZoomLevel::SolarSystem => {
            let map = world.current_system_map();
            map_renderer.render_system(canvas, map, state.position);
        }
        // ... etc for each level
    }
}
```

---

## Phase 5: World State Integration

### 5.1 Enhanced WorldState

**File:** `src/game/state.rs`

**Changes:**
```rust
pub struct WorldState {
    tick_count: u64,

    // Map data
    galaxy_map: GalaxyMap,
    loaded_systems: HashMap<EntityId, SolarSystemMap>,
    loaded_planets: HashMap<EntityId, PlanetMap>,
    loaded_regions: HashMap<EntityId, RegionMap>,
    loaded_areas: HashMap<EntityId, LocalAreaMap>,
    loaded_rooms: HashMap<EntityId, RoomMap>,

    // Generation seed
    seed: u64,
}
```

### 5.2 Navigation Methods

```rust
impl WorldState {
    // Movement
    pub fn can_move(&self, position: &Position, direction: Direction, level: ZoomLevel) -> bool;
    pub fn move_position(&self, position: &mut Position, direction: Direction, level: ZoomLevel) -> bool;

    // Entity interaction
    pub fn get_entity_at_position(&self, position: &Position, level: ZoomLevel) -> Option<EntityId>;
    pub fn can_enter_entity(&self, entity_id: EntityId, level: ZoomLevel) -> bool;

    // Map access
    pub fn get_galaxy_map(&self) -> &GalaxyMap;
    pub fn get_system_map(&mut self, system_id: EntityId) -> &SolarSystemMap;  // Lazy load
    pub fn get_planet_map(&mut self, planet_id: EntityId) -> &PlanetMap;

    // Map management (lazy loading)
    fn ensure_system_loaded(&mut self, system_id: EntityId);
    fn ensure_planet_loaded(&mut self, planet_id: EntityId);
}
```

---

## Phase 6: Game Loop Updates

### 6.1 Input Handling

**File:** `src/game/game_loop.rs`

```rust
fn handle_input(&mut self) -> Result<bool> {
    let action = self.input_handler.poll()?;

    match action {
        InputAction::Quit => return Ok(true),
        InputAction::TogglePause => self.time_controller.toggle_pause(),
        InputAction::IncreaseSpeed => self.time_controller.increase_speed(),
        InputAction::DecreaseSpeed => self.time_controller.decrease_speed(),

        // Navigation
        InputAction::MoveUp => {
            self.zoom_manager.move_in_direction(Direction::Up);
        }
        InputAction::MoveDown => {
            self.zoom_manager.move_in_direction(Direction::Down);
        }
        InputAction::MoveLeft => {
            self.zoom_manager.move_in_direction(Direction::Left);
        }
        InputAction::MoveRight => {
            self.zoom_manager.move_in_direction(Direction::Right);
        }

        // Enter entity (zoom in to specific entity)
        InputAction::Enter => {
            if self.zoom_manager.can_enter_current(&self.world_state) {
                self.zoom_manager.enter_current(&mut self.world_state);
            }
        }

        // Legacy zoom (for compatibility)
        InputAction::ZoomIn => self.zoom_manager.zoom_in(),
        InputAction::ZoomOut => self.zoom_manager.zoom_out(),

        InputAction::ToggleHelp | InputAction::None => {}
    }

    Ok(false)
}
```

---

## Implementation Timeline

### Sprint 1: Navigation Foundation (Week 1)
**Goal:** Player can move around with arrow keys

- [ ] Enhanced Position struct with grid coordinates
- [ ] Direction enum
- [ ] Arrow key input handling
- [ ] Basic movement logic in ZoomManager
- [ ] Update GameLoop to handle movement
- [ ] Tests for movement system

### Sprint 2: Galaxy & System Maps (Week 2)
**Goal:** Explorable galaxy and solar systems

- [ ] GalaxyMap structure and generator
- [ ] SolarSystemMap structure and generator
- [ ] Camera system for viewport management
- [ ] Map renderer for galaxy view
- [ ] Navigate galaxy, enter systems
- [ ] Render solar system view

### Sprint 3: Planet & Region Maps (Week 3)
**Goal:** Explorable planets with terrain

- [ ] PlanetMap with terrain generation
- [ ] RegionMap structure
- [ ] Terrain rendering with symbols
- [ ] Navigate planet surface
- [ ] Enter regions

### Sprint 4: Local Areas & Rooms (Week 4)
**Goal:** Complete navigation hierarchy

- [ ] LocalAreaMap with buildings
- [ ] RoomMap with interiors
- [ ] Full enter/exit navigation
- [ ] Complete zoom hierarchy
- [ ] Position persistence across zooms

### Sprint 5: Polish & Testing (Week 5)
**Goal:** Refinement and user experience

- [ ] Improve generation algorithms
- [ ] Add map variety and interesting features
- [ ] Performance optimization
- [ ] Comprehensive testing
- [ ] Documentation updates
- [ ] Help system updates

---

## Testing Strategy

### Unit Tests

**Position System:**
- Grid coordinate conversions
- Entity ID tracking
- Coordinate updates per level

**Map Generation:**
- Deterministic output (same seed = same map)
- Valid map structures
- Correct entity relationships
- No overlapping entities

**Navigation:**
- Movement respects boundaries
- Can't move into invalid positions
- Enter/exit maintains context

### Integration Tests

**Full Navigation Flow:**
- Start at galaxy
- Navigate to system
- Enter system
- Navigate to planet
- Enter planet
- Navigate to region
- Enter region
- Navigate to area
- Enter area
- Navigate to room
- Exit back through all levels

**Map Persistence:**
- Maps remain consistent
- Lazy loading works correctly
- Previously generated maps are reused

---

## Performance Considerations

### Memory Management
- **Lazy Loading:** Only generate/load maps when player visits
- **Unloading:** Unload distant maps from memory
- **Caching:** Keep recently visited maps cached

### Generation Speed
- **Fast Generation:** Galaxy should generate in < 1 second
- **Incremental:** Generate detailed maps on-demand
- **Asynchronous:** Consider background generation (future)

### Rendering Performance
- **Viewport Culling:** Only render visible entities
- **Entity Limits:** Cap visible entities per frame
- **Efficient Lookups:** Use spatial hashing for entity queries

---

## Success Criteria

✅ Player can navigate within each zoom level using arrow keys
✅ Arrow key movement is smooth and responsive
✅ Player can enter entities with Enter key
✅ Player can exit to parent level with zoom out
✅ Maps are procedurally generated and deterministic (same seed = same map)
✅ All 6 zoom levels have explorable content
✅ Position is maintained when zooming in/out
✅ Camera follows player smoothly
✅ Visual feedback for current position and available actions
✅ Performance remains stable (30+ FPS) with large maps
✅ Comprehensive test coverage for navigation and generation

---

## Future Enhancements (Post-MVP)

### Phase 2 Features
- **Named Locations:** Procedural name generation for all entities
- **Landmarks:** Unique, special locations at each zoom level
- **Interconnected Areas:** Multiple entrances/exits
- **Fog of War:** Explore to reveal, save explored state
- **Minimap:** Show context of current location
- **Search/Filter:** Find entities by name/type
- **Fast Travel:** Teleport to previously visited locations

### Phase 3 Features (Economic Integration)
- **Resources on Maps:** Distribute resources based on terrain
- **Trade Routes:** Connect areas with economic relationships
- **Population Centers:** Settlements with economic activity
- **Market Locations:** Specific rooms/areas for trading
- **NPC Locations:** Agents positioned on maps

---

## Related Documents

- [Foundation Plan](foundation-plan.md) - Core infrastructure (COMPLETE)
- Main README - Project overview
- Architecture documentation (future)

---

**Last Updated:** 2025-10-08
**Status:** 📋 Planning Complete - Ready for Implementation
