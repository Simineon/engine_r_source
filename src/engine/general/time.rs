use std::collections::HashMap;
use std::time::Instant;

pub struct Time {
    delta_time: f64,
    last_frame: Instant,
    timers: HashMap<String, f64>,
}

impl Time {
    pub fn new() -> Self {
        Self {
            delta_time: 0.0,
            last_frame: Instant::now(), // Because it's time's moment
            timers: HashMap::new(),
        }
    }

    // Updating Delta Time
    pub fn update_delta_time(&mut self) {
        let current_frame = Instant::now(); // Because it's time's moment right now(current frame)
        let duration = current_frame.duration_since(self.last_frame); // "duration_since" equal to (current_frame - last_frame)
        // We can't use (current_frame - last_frame) cuz it's Instant and we can't subtract one Instant from another(in rust)
        // sets
        self.delta_time = duration.as_secs_f64();
        self.last_frame = current_frame;

        for time_left in self.timers.values_mut() {
            if *time_left > 0.0 {
                *time_left -= self.delta_time;
            }
        }
    }

    pub fn get_delta_time(&self) -> f64 {
        self.delta_time
    }

    pub fn wait(&mut self, timer_name: &str, secs: f64) -> bool {
        let time_left = self.timers.entry(timer_name.to_string()).or_insert(secs);

        if *time_left <= 0.0 {
            *time_left = secs;
            true
        } else {
            false
        }
    }
}
