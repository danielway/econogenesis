use std::time::Duration;

pub struct WorldState {
    tick_count: u64,
}

impl WorldState {
    pub fn new() -> Self {
        Self {
            tick_count: 0,
        }
    }

    pub fn update(&mut self, _delta: Duration) {
        self.tick_count += 1;
    }

    pub fn tick_count(&self) -> u64 {
        self.tick_count
    }
}
