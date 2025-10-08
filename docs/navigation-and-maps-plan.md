# Navigation & Map System Plan

**Status:** Phase 1 Complete - In Development
**Last Updated:** 2025-10-08

## Overview

Full navigation and procedural generation across all 6 zoom levels:

**Galaxy → Solar System → Planet → Region → Local Area → Room**

**Features:**
- Arrow key navigation at each zoom level
- Zoom into entities (systems, planets, regions, areas, rooms)
- Zoom out to parent levels maintaining context
- Procedurally generated, persistent worlds
- Position and navigation feedback

## Phase 1: Navigation Infrastructure (Foundation) ✅ COMPLETE

**Completed:** 2025-10-08
**Files Modified:**
- `src/zoom/manager.rs` - Position, Direction, ZoomManager
- `src/input/handler.rs` - InputAction enum
- `src/game/game_loop.rs` - Input handling
- `src/zoom/mod.rs` - Module exports

### 1.1 Enhanced Position System ✅

**Goal:** Enable player to navigate within each zoom level with proper tracking.

**Status:** ✅ Implemented in `src/zoom/manager.rs:72-154`

**Implementation:** Fully implemented with entity ID tracking, grid coordinates for all 6 zoom levels, and methods for coordinate access and mutation. Includes legacy floating-point coordinates for backward compatibility.

**Key Methods:**
- ✅ `coords_for_level(&self, level: ZoomLevel) -> (i32, i32)`
- ✅ `set_coords_for_level(&mut self, level: ZoomLevel, coords: (i32, i32))`
- ✅ `current_entity_id(&self, level: ZoomLevel) -> Option<EntityId>`

**Tests:** 4 tests covering coordinate access, mutation, and entity ID tracking

### 1.2 Direction Enum ✅

**Status:** ✅ Implemented in `src/zoom/manager.rs:15-32`

**Implementation:** Complete with Up, Down, Left, Right variants and `to_offset()` method for converting directions to (dx, dy) coordinate offsets.

**Tests:** 1 test verifying all direction offsets

### 1.3 ZoomManager Navigation ✅

**Status:** ✅ Implemented in `src/zoom/manager.rs:195-210`

**Implemented Methods:**
- ✅ `move_in_direction(&mut self, direction: Direction) -> bool` - Movement within current zoom level
- ✅ `position_mut(&mut self) -> &mut Position` - Mutable access to position

**Not Yet Implemented (Phase 2):**
- ⏳ `can_enter_current(&self, world_state: &WorldState) -> bool` - Requires map data
- ⏳ `enter_current(&mut self, world_state: &WorldState) -> bool` - Requires map data
- ⏳ `exit_to_parent(&mut self) -> bool` - Requires parent tracking

**Tests:** 4 tests covering basic movement, multi-level movement, and negative coordinates

### 1.4 Input Actions for Navigation ✅

**Status:** ✅ Implemented in `src/input/handler.rs:5-56`

**New InputAction Variants:** MoveUp, MoveDown, MoveLeft, MoveRight, Enter

**Key Mappings:** All arrow keys and Enter key properly mapped

**Game Loop Integration:** ✅ Complete in `src/game/game_loop.rs:62-98`

**Help System:** ✅ Updated with new controls

**Status Bar:** ✅ Updated to show navigation hints

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

Galaxy: `*` star system, `·` empty space, `@` player
Solar System: `☉` star, `o/O` planets, `@` player ship
Planet: `~` ocean, `^` mountains, `"` plains, `♣` forest, `#` desert, `*` ice, `@` player
Region: ASCII settlements, terrain symbols, roads
Area/Room: `▓` walls, `░` floors, `├─┤` doors, `@` player

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

### Sprint 1: Navigation Foundation (Week 1) ✅ COMPLETE
**Goal:** Player can move around with arrow keys
**Completed:** 2025-10-08

- [x] Enhanced Position struct with grid coordinates
- [x] Direction enum
- [x] Arrow key input handling
- [x] Basic movement logic in ZoomManager
- [x] Update GameLoop to handle movement
- [x] Tests for movement system

**Results:**
- 7 new tests added (all passing)
- Total test count: 25 tests passing
- Position system supports all 6 zoom levels
- Arrow keys fully functional
- Coordinates display in UI

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

**Position System:** ✅ Complete
- ✅ Grid coordinate conversions
- ✅ Entity ID tracking
- ✅ Coordinate updates per level

**Map Generation:**
- Deterministic output (same seed = same map)
- Valid map structures
- Correct entity relationships
- No overlapping entities

**Navigation:** ✅ Phase 1 Complete
- ✅ Movement works in all directions
- ✅ Independent movement at each zoom level
- ✅ Negative coordinates supported
- ⏳ Movement respects boundaries (Phase 2 - requires maps)
- ⏳ Can't move into invalid positions (Phase 2 - requires maps)
- ⏳ Enter/exit maintains context (Phase 2 - requires maps)

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

### Phase 1 (Complete)
- Player can navigate with arrow keys at each zoom level
- Position maintained independently at each level
- Visual feedback for current position
- 7 navigation tests passing

### Phase 2+ (Pending)
- Enter entities with Enter key (requires maps)
- Exit to parent level with zoom out (requires parent tracking)
- Procedurally generated deterministic maps
- All 6 zoom levels have explorable content
- Camera follows player
- Stable performance (30+ FPS)
- Map generation test coverage

## Future Enhancements

Post-MVP features:
- Named locations with procedural generation
- Unique landmarks at each zoom level
- Multiple entrances/exits between areas
- Fog of war
- Minimap
- Search/filter entities
- Fast travel

Economic integration:
- Resources distributed based on terrain
- Trade routes connecting areas
- Population centers with economic activity
- Market locations
- Positioned NPCs/agents

## Progress Summary

**Phase 1: Navigation Infrastructure** - Complete (2025-10-08)
- Core navigation features implemented
- 7 new tests, 25 total passing
- Arrow key movement at all zoom levels
- Coordinate display and help system

**Next: Phase 2 - Map Data Structures**
- GalaxyMap and SolarSystemMap structures
- Procedural generation
- Camera system
