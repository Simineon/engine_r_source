use crate::engine::general::inputing::keys::Key;
use std::collections::HashMap;
use std::time::{Duration, Instant};

pub struct Input {
    current_keys: HashMap<Key, bool>,         // Current frame key states
    previous_keys: HashMap<Key, bool>,        // Previous frame key states (for detecting taps)
    key_press_times: HashMap<Key, Instant>,   // When each key was pressed
    key_release_times: HashMap<Key, Instant>, // When each key was released
    pressed_this_frame: HashMap<Key, bool>,   // Whether key was pressed this frame
}

impl Input {
    pub fn new() -> Self {
        Self {
            current_keys: HashMap::new(),
            previous_keys: HashMap::new(),
            key_press_times: HashMap::new(),
            key_release_times: HashMap::new(),
            pressed_this_frame: HashMap::new(),
        }
    }

    // Call this at the start of each frame to update input state
    pub fn update(&mut self) {
        for (_, pressed) in self.pressed_this_frame.iter_mut() {
            *pressed = false;
        }

        // Store current state as previous
        self.previous_keys = self.current_keys.clone();
    }

    // Handle a key press event
    pub fn on_key_press(&mut self, key: Key) {
        self.current_keys.insert(key, true);
        self.key_press_times.insert(key, Instant::now());
        self.pressed_this_frame.insert(key, true);
    }

    // Handle a key release event
    pub fn on_key_release(&mut self, key: Key) {
        self.current_keys.insert(key, false);
        self.key_release_times.insert(key, Instant::now());
        self.pressed_this_frame.insert(key, false);
    }

    // Check if a key is currently being held down
    pub fn is_key_pressed(&self, key: Key) -> bool {
        *self.current_keys.get(&key).unwrap_or(&false)
    }

    // Check if a key was released this frame
    pub fn is_key_released(&self, key: Key) -> bool {
        let was_pressed = *self.previous_keys.get(&key).unwrap_or(&false);
        let is_pressed = *self.current_keys.get(&key).unwrap_or(&false);
        was_pressed && !is_pressed
    }

    // Get how long a key has been held down (returns None if not pressed)
    pub fn get_key_duration(&self, key: Key) -> Option<Duration> {
        if !self.is_key_pressed(key) {
            return None;
        }
        self.key_press_times.get(&key).map(|time| time.elapsed())
    }

    // Get how long a key has been released (returns None if still pressed)
    pub fn get_key_release_duration(&self, key: Key) -> Option<Duration> {
        if self.is_key_pressed(key) {
            return None;
        }
        self.key_release_times.get(&key).map(|time| time.elapsed())
    }
}
