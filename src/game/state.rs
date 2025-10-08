use crate::zoom::{Position, ZoomLevel};
use std::collections::HashMap;
use std::time::Duration;

pub type EntityId = u64;

#[derive(Debug, Clone)]
pub struct GalaxyState {
    pub name: String,
    #[allow(dead_code)]
    pub star_count: u64,
}

#[derive(Debug, Clone)]
pub struct SolarSystemState {
    #[allow(dead_code)]
    pub id: EntityId,
    pub name: String,
    #[allow(dead_code)]
    pub planet_count: u32,
}

#[derive(Debug, Clone)]
pub struct PlanetState {
    #[allow(dead_code)]
    pub id: EntityId,
    pub name: String,
    #[allow(dead_code)]
    pub population: u64,
}

#[derive(Debug, Clone)]
pub struct RegionState {
    #[allow(dead_code)]
    pub id: EntityId,
    pub name: String,
    #[allow(dead_code)]
    pub terrain_type: String,
}

#[derive(Debug, Clone)]
pub struct LocalAreaState {
    #[allow(dead_code)]
    pub id: EntityId,
    pub name: String,
    #[allow(dead_code)]
    pub building_count: u32,
}

#[derive(Debug, Clone)]
pub struct RoomState {
    #[allow(dead_code)]
    pub id: EntityId,
    pub name: String,
    #[allow(dead_code)]
    pub room_type: String,
}

pub struct WorldState {
    tick_count: u64,
    #[allow(dead_code)]
    player_position: Position,
    galaxy: GalaxyState,
    systems: HashMap<EntityId, SolarSystemState>,
    planets: HashMap<EntityId, PlanetState>,
    regions: HashMap<EntityId, RegionState>,
    areas: HashMap<EntityId, LocalAreaState>,
    rooms: HashMap<EntityId, RoomState>,
}

impl WorldState {
    pub fn new() -> Self {
        let mut state = Self {
            tick_count: 0,
            player_position: Position::new(),
            galaxy: GalaxyState {
                name: String::from("Andromeda Prime"),
                star_count: 1_000_000_000,
            },
            systems: HashMap::new(),
            planets: HashMap::new(),
            regions: HashMap::new(),
            areas: HashMap::new(),
            rooms: HashMap::new(),
        };

        state.initialize_sample_data();
        state
    }

    fn initialize_sample_data(&mut self) {
        self.systems.insert(
            1,
            SolarSystemState {
                id: 1,
                name: String::from("Sol System"),
                planet_count: 8,
            },
        );

        self.planets.insert(
            1,
            PlanetState {
                id: 1,
                name: String::from("Terra"),
                population: 7_800_000_000,
            },
        );

        self.regions.insert(
            1,
            RegionState {
                id: 1,
                name: String::from("Northern Highlands"),
                terrain_type: String::from("Mountains"),
            },
        );

        self.areas.insert(
            1,
            LocalAreaState {
                id: 1,
                name: String::from("Market District"),
                building_count: 47,
            },
        );

        self.rooms.insert(
            1,
            RoomState {
                id: 1,
                name: String::from("Trading Hall"),
                room_type: String::from("Commercial"),
            },
        );
    }

    pub fn update(&mut self, _delta: Duration) {
        self.tick_count += 1;
    }

    pub fn tick_count(&self) -> u64 {
        self.tick_count
    }

    #[allow(dead_code)]
    pub fn player_position(&self) -> &Position {
        &self.player_position
    }

    #[allow(dead_code)]
    pub fn galaxy(&self) -> &GalaxyState {
        &self.galaxy
    }

    pub fn get_system(&self, id: EntityId) -> Option<&SolarSystemState> {
        self.systems.get(&id)
    }

    pub fn get_planet(&self, id: EntityId) -> Option<&PlanetState> {
        self.planets.get(&id)
    }

    pub fn get_region(&self, id: EntityId) -> Option<&RegionState> {
        self.regions.get(&id)
    }

    pub fn get_area(&self, id: EntityId) -> Option<&LocalAreaState> {
        self.areas.get(&id)
    }

    pub fn get_room(&self, id: EntityId) -> Option<&RoomState> {
        self.rooms.get(&id)
    }

    pub fn get_current_entity_name(&self, zoom_level: ZoomLevel) -> String {
        match zoom_level {
            ZoomLevel::Galaxy => self.galaxy.name.clone(),
            ZoomLevel::SolarSystem => self
                .get_system(1)
                .map(|s| s.name.clone())
                .unwrap_or_else(|| String::from("Unknown System")),
            ZoomLevel::Planet => self
                .get_planet(1)
                .map(|p| p.name.clone())
                .unwrap_or_else(|| String::from("Unknown Planet")),
            ZoomLevel::Region => self
                .get_region(1)
                .map(|r| r.name.clone())
                .unwrap_or_else(|| String::from("Unknown Region")),
            ZoomLevel::LocalArea => self
                .get_area(1)
                .map(|a| a.name.clone())
                .unwrap_or_else(|| String::from("Unknown Area")),
            ZoomLevel::Room => self
                .get_room(1)
                .map(|r| r.name.clone())
                .unwrap_or_else(|| String::from("Unknown Room")),
        }
    }

    pub fn entity_count(&self) -> usize {
        1 + self.systems.len()
            + self.planets.len()
            + self.regions.len()
            + self.areas.len()
            + self.rooms.len()
    }
}

impl Default for WorldState {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_world_state_initialization() {
        let state = WorldState::new();
        assert_eq!(state.tick_count(), 0);
        assert_eq!(state.galaxy().name, "Andromeda Prime");
        assert_eq!(state.entity_count(), 6);
    }

    #[test]
    fn test_world_state_update() {
        let mut state = WorldState::new();
        state.update(Duration::from_secs(1));
        assert_eq!(state.tick_count(), 1);
        state.update(Duration::from_secs(1));
        assert_eq!(state.tick_count(), 2);
    }

    #[test]
    fn test_sample_data_exists() {
        let state = WorldState::new();
        assert!(state.get_system(1).is_some());
        assert!(state.get_planet(1).is_some());
        assert!(state.get_region(1).is_some());
        assert!(state.get_area(1).is_some());
        assert!(state.get_room(1).is_some());
    }

    #[test]
    fn test_current_entity_name() {
        let state = WorldState::new();
        assert_eq!(
            state.get_current_entity_name(ZoomLevel::Galaxy),
            "Andromeda Prime"
        );
        assert_eq!(
            state.get_current_entity_name(ZoomLevel::SolarSystem),
            "Sol System"
        );
        assert_eq!(state.get_current_entity_name(ZoomLevel::Planet), "Terra");
        assert_eq!(
            state.get_current_entity_name(ZoomLevel::Region),
            "Northern Highlands"
        );
        assert_eq!(
            state.get_current_entity_name(ZoomLevel::LocalArea),
            "Market District"
        );
        assert_eq!(
            state.get_current_entity_name(ZoomLevel::Room),
            "Trading Hall"
        );
    }

    #[test]
    fn test_player_position() {
        let state = WorldState::new();
        let pos = state.player_position();
        assert_eq!(pos.galaxy_coords, (0, 0));
    }
}
