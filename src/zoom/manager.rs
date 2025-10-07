use std::fmt;

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
pub struct Position {
    pub galaxy_coords: (f64, f64, f64),
    pub system_coords: (f64, f64, f64),
    pub planet_coords: (f64, f64, f64),
    pub local_coords: (f64, f64, f64),
}

impl Default for Position {
    fn default() -> Self {
        Self {
            galaxy_coords: (0.0, 0.0, 0.0),
            system_coords: (0.0, 0.0, 0.0),
            planet_coords: (0.0, 0.0, 0.0),
            local_coords: (0.0, 0.0, 0.0),
        }
    }
}

impl Position {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn coords_for_level(&self, level: ZoomLevel) -> (f64, f64, f64) {
        match level {
            ZoomLevel::Galaxy => self.galaxy_coords,
            ZoomLevel::SolarSystem => self.system_coords,
            ZoomLevel::Planet | ZoomLevel::Region => self.planet_coords,
            ZoomLevel::LocalArea | ZoomLevel::Room => self.local_coords,
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
}
