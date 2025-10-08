use std::fmt;

use crate::game::state::EntityId;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum ZoomLevel {
    Room,
    LocalArea,
    Region,
    Planet,
    SolarSystem,
    Galaxy,
}

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

impl ZoomLevel {
    pub fn zoom_in(self) -> Option<Self> {
        match self {
            ZoomLevel::Galaxy => Some(ZoomLevel::SolarSystem),
            ZoomLevel::SolarSystem => Some(ZoomLevel::Planet),
            ZoomLevel::Planet => Some(ZoomLevel::Region),
            ZoomLevel::Region => Some(ZoomLevel::LocalArea),
            ZoomLevel::LocalArea => Some(ZoomLevel::Room),
            ZoomLevel::Room => None,
        }
    }

    pub fn zoom_out(self) -> Option<Self> {
        match self {
            ZoomLevel::Room => Some(ZoomLevel::LocalArea),
            ZoomLevel::LocalArea => Some(ZoomLevel::Region),
            ZoomLevel::Region => Some(ZoomLevel::Planet),
            ZoomLevel::Planet => Some(ZoomLevel::SolarSystem),
            ZoomLevel::SolarSystem => Some(ZoomLevel::Galaxy),
            ZoomLevel::Galaxy => None,
        }
    }
}

impl fmt::Display for ZoomLevel {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ZoomLevel::Room => write!(f, "Room"),
            ZoomLevel::LocalArea => write!(f, "Local Area"),
            ZoomLevel::Region => write!(f, "Region"),
            ZoomLevel::Planet => write!(f, "Planet"),
            ZoomLevel::SolarSystem => write!(f, "Solar System"),
            ZoomLevel::Galaxy => write!(f, "Galaxy"),
        }
    }
}

#[derive(Debug, Clone, Copy)]
#[derive(Default)]
pub struct Position {
    // Entity ID tracking - which specific entity at each level
    pub current_system_id: Option<EntityId>,
    pub current_planet_id: Option<EntityId>,
    pub current_region_id: Option<EntityId>,
    pub current_area_id: Option<EntityId>,
    pub current_room_id: Option<EntityId>,

    // Grid coordinates for spatial navigation (integer-based)
    pub galaxy_coords: (i32, i32),
    pub system_coords: (i32, i32),
    pub planet_coords: (i32, i32),
    pub region_coords: (i32, i32),
    pub area_coords: (i32, i32),
    pub room_coords: (i32, i32),
}


impl Position {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn coords_for_level(&self, level: ZoomLevel) -> (i32, i32) {
        match level {
            ZoomLevel::Galaxy => self.galaxy_coords,
            ZoomLevel::SolarSystem => self.system_coords,
            ZoomLevel::Planet => self.planet_coords,
            ZoomLevel::Region => self.region_coords,
            ZoomLevel::LocalArea => self.area_coords,
            ZoomLevel::Room => self.room_coords,
        }
    }

    pub fn set_coords_for_level(&mut self, level: ZoomLevel, coords: (i32, i32)) {
        match level {
            ZoomLevel::Galaxy => self.galaxy_coords = coords,
            ZoomLevel::SolarSystem => self.system_coords = coords,
            ZoomLevel::Planet => self.planet_coords = coords,
            ZoomLevel::Region => self.region_coords = coords,
            ZoomLevel::LocalArea => self.area_coords = coords,
            ZoomLevel::Room => self.room_coords = coords,
        }
    }

    pub fn current_entity_id(&self, level: ZoomLevel) -> Option<EntityId> {
        match level {
            ZoomLevel::Galaxy => None,
            ZoomLevel::SolarSystem => self.current_system_id,
            ZoomLevel::Planet => self.current_planet_id,
            ZoomLevel::Region => self.current_region_id,
            ZoomLevel::LocalArea => self.current_area_id,
            ZoomLevel::Room => self.current_room_id,
        }
    }
}

pub struct ZoomManager {
    current_level: ZoomLevel,
    position: Position,
}

impl ZoomManager {
    pub fn new() -> Self {
        Self {
            current_level: ZoomLevel::Galaxy,
            position: Position::new(),
        }
    }

    pub fn current_level(&self) -> ZoomLevel {
        self.current_level
    }

    pub fn position(&self) -> &Position {
        &self.position
    }

    pub fn zoom_in(&mut self) -> bool {
        if let Some(new_level) = self.current_level.zoom_in() {
            self.current_level = new_level;
            true
        } else {
            false
        }
    }

    pub fn zoom_out(&mut self) -> bool {
        if let Some(new_level) = self.current_level.zoom_out() {
            self.current_level = new_level;
            true
        } else {
            false
        }
    }

    /// Move in a direction within the current zoom level
    /// Returns true if the movement was successful
    pub fn move_in_direction(&mut self, direction: Direction) -> bool {
        let current_coords = self.position.coords_for_level(self.current_level);
        let offset = direction.to_offset();
        let new_coords = (current_coords.0 + offset.0, current_coords.1 + offset.1);

        // For now, allow unlimited movement (will be constrained by map boundaries later)
        self.position
            .set_coords_for_level(self.current_level, new_coords);
        true
    }

    /// Get mutable access to position for advanced operations
    pub fn position_mut(&mut self) -> &mut Position {
        &mut self.position
    }
}

impl Default for ZoomManager {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn zoom_in_transitions() {
        assert_eq!(ZoomLevel::Galaxy.zoom_in(), Some(ZoomLevel::SolarSystem));
        assert_eq!(ZoomLevel::SolarSystem.zoom_in(), Some(ZoomLevel::Planet));
        assert_eq!(ZoomLevel::Planet.zoom_in(), Some(ZoomLevel::Region));
        assert_eq!(ZoomLevel::Region.zoom_in(), Some(ZoomLevel::LocalArea));
        assert_eq!(ZoomLevel::LocalArea.zoom_in(), Some(ZoomLevel::Room));
        assert_eq!(ZoomLevel::Room.zoom_in(), None);
    }

    #[test]
    fn zoom_out_transitions() {
        assert_eq!(ZoomLevel::Room.zoom_out(), Some(ZoomLevel::LocalArea));
        assert_eq!(ZoomLevel::LocalArea.zoom_out(), Some(ZoomLevel::Region));
        assert_eq!(ZoomLevel::Region.zoom_out(), Some(ZoomLevel::Planet));
        assert_eq!(ZoomLevel::Planet.zoom_out(), Some(ZoomLevel::SolarSystem));
        assert_eq!(ZoomLevel::SolarSystem.zoom_out(), Some(ZoomLevel::Galaxy));
        assert_eq!(ZoomLevel::Galaxy.zoom_out(), None);
    }

    #[test]
    fn zoom_manager_starts_at_galaxy() {
        let manager = ZoomManager::new();
        assert_eq!(manager.current_level(), ZoomLevel::Galaxy);
    }

    #[test]
    fn zoom_manager_can_zoom_in() {
        let mut manager = ZoomManager::new();
        assert!(manager.zoom_in());
        assert_eq!(manager.current_level(), ZoomLevel::SolarSystem);
        assert!(manager.zoom_in());
        assert_eq!(manager.current_level(), ZoomLevel::Planet);
    }

    #[test]
    fn zoom_manager_can_zoom_out() {
        let mut manager = ZoomManager::new();
        manager.zoom_in();
        manager.zoom_in();
        assert!(manager.zoom_out());
        assert_eq!(manager.current_level(), ZoomLevel::SolarSystem);
    }

    #[test]
    fn zoom_manager_cannot_zoom_beyond_limits() {
        let mut manager = ZoomManager::new();
        assert!(!manager.zoom_out());
        assert_eq!(manager.current_level(), ZoomLevel::Galaxy);

        for _ in 0..6 {
            manager.zoom_in();
        }
        assert_eq!(manager.current_level(), ZoomLevel::Room);
        assert!(!manager.zoom_in());
    }

    #[test]
    fn zoom_levels_ordered_correctly() {
        assert!(ZoomLevel::Room < ZoomLevel::LocalArea);
        assert!(ZoomLevel::LocalArea < ZoomLevel::Region);
        assert!(ZoomLevel::Region < ZoomLevel::Planet);
        assert!(ZoomLevel::Planet < ZoomLevel::SolarSystem);
        assert!(ZoomLevel::SolarSystem < ZoomLevel::Galaxy);
    }

    #[test]
    fn direction_to_offset() {
        assert_eq!(Direction::Up.to_offset(), (0, -1));
        assert_eq!(Direction::Down.to_offset(), (0, 1));
        assert_eq!(Direction::Left.to_offset(), (-1, 0));
        assert_eq!(Direction::Right.to_offset(), (1, 0));
    }

    #[test]
    fn position_coords_for_level() {
        let mut pos = Position::new();
        pos.galaxy_coords = (5, 10);
        pos.system_coords = (3, 7);
        pos.planet_coords = (12, 8);

        assert_eq!(pos.coords_for_level(ZoomLevel::Galaxy), (5, 10));
        assert_eq!(pos.coords_for_level(ZoomLevel::SolarSystem), (3, 7));
        assert_eq!(pos.coords_for_level(ZoomLevel::Planet), (12, 8));
    }

    #[test]
    fn position_set_coords_for_level() {
        let mut pos = Position::new();
        pos.set_coords_for_level(ZoomLevel::Galaxy, (10, 20));
        pos.set_coords_for_level(ZoomLevel::SolarSystem, (5, 15));

        assert_eq!(pos.galaxy_coords, (10, 20));
        assert_eq!(pos.system_coords, (5, 15));
    }

    #[test]
    fn position_entity_id_tracking() {
        let mut pos = Position::new();
        assert_eq!(pos.current_entity_id(ZoomLevel::Galaxy), None);
        assert_eq!(pos.current_entity_id(ZoomLevel::SolarSystem), None);

        pos.current_system_id = Some(42);
        pos.current_planet_id = Some(100);

        assert_eq!(pos.current_entity_id(ZoomLevel::SolarSystem), Some(42));
        assert_eq!(pos.current_entity_id(ZoomLevel::Planet), Some(100));
    }

    #[test]
    fn zoom_manager_movement() {
        let mut manager = ZoomManager::new();
        assert_eq!(manager.position().galaxy_coords, (0, 0));

        // Move right
        manager.move_in_direction(Direction::Right);
        assert_eq!(manager.position().galaxy_coords, (1, 0));

        // Move down
        manager.move_in_direction(Direction::Down);
        assert_eq!(manager.position().galaxy_coords, (1, 1));

        // Move left
        manager.move_in_direction(Direction::Left);
        assert_eq!(manager.position().galaxy_coords, (0, 1));

        // Move up
        manager.move_in_direction(Direction::Up);
        assert_eq!(manager.position().galaxy_coords, (0, 0));
    }

    #[test]
    fn zoom_manager_movement_different_levels() {
        let mut manager = ZoomManager::new();

        // Move at galaxy level
        manager.move_in_direction(Direction::Right);
        assert_eq!(manager.position().galaxy_coords, (1, 0));

        // Zoom into solar system
        manager.zoom_in();
        assert_eq!(manager.current_level(), ZoomLevel::SolarSystem);

        // Movement at solar system level shouldn't affect galaxy coords
        manager.move_in_direction(Direction::Down);
        assert_eq!(manager.position().system_coords, (0, 1));
        assert_eq!(manager.position().galaxy_coords, (1, 0)); // unchanged

        // Zoom into planet
        manager.zoom_in();
        assert_eq!(manager.current_level(), ZoomLevel::Planet);

        // Movement at planet level
        manager.move_in_direction(Direction::Left);
        manager.move_in_direction(Direction::Left);
        assert_eq!(manager.position().planet_coords, (-2, 0));
        assert_eq!(manager.position().system_coords, (0, 1)); // unchanged
        assert_eq!(manager.position().galaxy_coords, (1, 0)); // unchanged
    }

    #[test]
    fn zoom_manager_allows_negative_coordinates() {
        let mut manager = ZoomManager::new();

        // Move left from origin - should allow negative coordinates
        manager.move_in_direction(Direction::Left);
        assert_eq!(manager.position().galaxy_coords, (-1, 0));

        // Move up from origin
        manager.move_in_direction(Direction::Up);
        assert_eq!(manager.position().galaxy_coords, (-1, -1));
    }
}
