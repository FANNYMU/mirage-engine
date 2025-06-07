use std::time::{Duration, Instant};

pub struct DeltaTime {
    last_update: Instant,
    delta: Duration,
    fixed_timestep: Option<Duration>,
}

impl DeltaTime {
    pub fn new() -> Self {
        Self {
            last_update: Instant::now(),
            delta: Duration::from_secs(0),
            fixed_timestep: None,
        }
    }

    pub fn with_fixed_timestep(fps: u32) -> Self {
        let fixed_dt = Duration::from_secs_f64(1.0 / fps as f64);
        Self {
            last_update: Instant::now(),
            delta: fixed_dt,
            fixed_timestep: Some(fixed_dt),
        }
    }

    pub fn update(&mut self) -> f32 {
        let now = Instant::now();
        self.delta = now - self.last_update;
        self.last_update = now;
        
        if let Some(fixed) = self.fixed_timestep {
            return fixed.as_secs_f32();
        }
        
        self.delta.as_secs_f32()
    }

    pub fn delta_seconds(&self) -> f32 {
        self.delta.as_secs_f32()
    }

    pub fn delta_millis(&self) -> u128 {
        self.delta.as_millis()
    }

    pub fn fps(&self) -> f32 {
        1.0 / self.delta_seconds()
    }
} 