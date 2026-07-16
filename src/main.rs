//! Main File
//! main.rs
//!
//! - Ther Main file is file which set everything from engine in unit,
//! - The Main file which containts Scenes, Scripts Structure headers,
//! - The Main file launch all application,
//! - Containts imports.

mod engine;
use crate::engine::general::inputing::keys::Key;
use crate::engine::general::objects2d::sprite::Sprite;
use engine::app::Component;
use engine::app::GameApp;
use engine::general::inputing::input::Input;
use engine::general::scene::scene::Scene;
use engine::general::time::Time;
use std::sync::{Arc, Mutex};

pub struct PlayerController {
    speed: f32,
    player_sprite_index: usize,
}

impl PlayerController {
    pub fn new(player_sprite_index: usize) -> Self {
        Self {
            speed: 3.0,
            player_sprite_index,
        }
    }
}

impl Component for PlayerController {
    fn start(&mut self) {
        println!("Player started!");
    }

    fn update(&mut self, input: &Input, time: &Time, sprites: &mut Vec<Sprite>) {
        let delta_time = time.get_delta_time() as f32;

        let sprite = match sprites.get_mut(self.player_sprite_index) {
            Some(s) => s,
            None => return,
        };

        let mut dx = 0.0;
        let mut dy = 0.0;

        if input.is_key_pressed(Key::W) {
            dy += self.speed * delta_time;
        }
        if input.is_key_pressed(Key::S) {
            dy -= self.speed * delta_time;
        }
        if input.is_key_pressed(Key::A) {
            dx -= self.speed * delta_time;
        }
        if input.is_key_pressed(Key::D) {
            dx += self.speed * delta_time;
        }

        if dx != 0.0 || dy != 0.0 {
            sprite.x += dx;
            sprite.y += dy;
        }
    }
}

fn main() {
    let mut app = GameApp::new("Engine");

    let mut level_1 = Scene::new(1, "Level 1".to_string());

    let player_sprite = Sprite {
        x: 0.0,
        y: 0.0,
        width: 0.5,
        height: 1.0,
        texture_name: "rs".to_string(),
    };

    let p_idx = level_1.add_sprite(player_sprite);

    let controller = Box::new(PlayerController::new(p_idx));
    level_1.add_component(controller);

    let mut level_2 = Scene::new(2, "Level 2".to_string());

    // Register scenes in engine
    app.scene_adaptor.add_scene(level_1);
    app.scene_adaptor.add_scene(level_2);

    //app.scene_adaptor.change_scene(2);

    app.run();
}
