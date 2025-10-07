use std::time::{Duration, Instant};

pub struct TimeController {
    is_paused: bool,
    speed_multiplier: f64,
    simulation_time: Duration,
    last_update: Instant,
    target_fps: u32,
}

impl TimeController {
    pub fn new(target_fps: u32) -> Self {
        Self {
            is_paused: true,
            speed_multiplier: 1.0,
            simulation_time: Duration::ZERO,
            last_update: Instant::now(),
            target_fps,
        }
    }

    pub fn is_paused(&self) -> bool {
        self.is_paused
    }

    pub fn toggle_pause(&mut self) {
        self.is_paused = !self.is_paused;
        self.last_update = Instant::now();
    }

    pub fn speed_multiplier(&self) -> f64 {
        self.speed_multiplier
    }

    pub fn increase_speed(&mut self) {
        self.speed_multiplier = match self.speed_multiplier {
            x if x < 0.5 => 0.5,
            x if x < 1.0 => 1.0,
            x if x < 2.0 => 2.0,
            x if x < 5.0 => 5.0,
            x if x < 10.0 => 10.0,
            x if x < 20.0 => 20.0,
            _ => 50.0,
        };
    }

    pub fn decrease_speed(&mut self) {
        self.speed_multiplier = match self.speed_multiplier {
            x if x <= 0.5 => 0.1,
            x if x <= 1.0 => 0.5,
            x if x <= 2.0 => 1.0,
            x if x <= 5.0 => 2.0,
            x if x <= 10.0 => 5.0,
            x if x <= 20.0 => 10.0,
            _ => 20.0,
        };
    }

    pub fn simulation_time(&self) -> Duration {
        self.simulation_time
    }

    pub fn delta_time(&self) -> Duration {
        if self.is_paused {
            Duration::ZERO
        } else {
            let real_delta = self.last_update.elapsed();
            let scaled_seconds = real_delta.as_secs_f64() * self.speed_multiplier;
            Duration::from_secs_f64(scaled_seconds)
        }
    }

    pub fn step(&mut self) -> Duration {
        let delta = self.delta_time();

        if !self.is_paused {
            self.simulation_time += delta;
        }

        self.last_update = Instant::now();
        delta
    }

    pub fn target_frame_duration(&self) -> Duration {
        Duration::from_secs_f64(1.0 / self.target_fps as f64)
    }

    pub fn format_time(&self) -> String {
        let total_secs = self.simulation_time.as_secs();
        let days = total_secs / 86400;
        let hours = (total_secs % 86400) / 3600;
        let minutes = (total_secs % 3600) / 60;
        let seconds = total_secs % 60;

        if days > 0 {
            format!("{}d {}h {}m {}s", days, hours, minutes, seconds)
        } else if hours > 0 {
            format!("{}h {}m {}s", hours, minutes, seconds)
        } else if minutes > 0 {
            format!("{}m {}s", minutes, seconds)
        } else {
            format!("{}s", seconds)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::thread;

    #[test]
    fn test_starts_paused() {
        let controller = TimeController::new(60);
        assert!(controller.is_paused());
    }

    #[test]
    fn test_toggle_pause() {
        let mut controller = TimeController::new(60);
        assert!(controller.is_paused());

        controller.toggle_pause();
        assert!(!controller.is_paused());

        controller.toggle_pause();
        assert!(controller.is_paused());
    }

    #[test]
    fn test_speed_increase() {
        let mut controller = TimeController::new(60);

        controller.speed_multiplier = 0.1;
        controller.increase_speed();
        assert_eq!(controller.speed_multiplier(), 0.5);

        controller.increase_speed();
        assert_eq!(controller.speed_multiplier(), 1.0);

        controller.increase_speed();
        assert_eq!(controller.speed_multiplier(), 2.0);
    }

    #[test]
    fn test_speed_decrease() {
        let mut controller = TimeController::new(60);

        controller.speed_multiplier = 2.0;
        controller.decrease_speed();
        assert_eq!(controller.speed_multiplier(), 1.0);

        controller.decrease_speed();
        assert_eq!(controller.speed_multiplier(), 0.5);

        controller.decrease_speed();
        assert_eq!(controller.speed_multiplier(), 0.1);
    }

    #[test]
    fn test_paused_time_doesnt_advance() {
        let mut controller = TimeController::new(60);
        assert!(controller.is_paused());

        thread::sleep(Duration::from_millis(50));
        let delta = controller.step();

        assert_eq!(delta, Duration::ZERO);
        assert_eq!(controller.simulation_time(), Duration::ZERO);
    }

    #[test]
    fn test_format_time() {
        let mut controller = TimeController::new(60);

        controller.simulation_time = Duration::from_secs(45);
        assert_eq!(controller.format_time(), "45s");

        controller.simulation_time = Duration::from_secs(125);
        assert_eq!(controller.format_time(), "2m 5s");

        controller.simulation_time = Duration::from_secs(3665);
        assert_eq!(controller.format_time(), "1h 1m 5s");

        controller.simulation_time = Duration::from_secs(90061);
        assert_eq!(controller.format_time(), "1d 1h 1m 1s");
    }
}
