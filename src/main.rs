//! Main File
//! main.rs
//!
//! - Ther Main file is file which set everything from engine in unit,
//! - The Main file which containts Scenes, Scripts Structure headers,
//! - The Main file launch all application,
//! - Containts imports.

mod engine;
use crate::engine::general::camera::Camera;
use crate::engine::general::inputing::keys::Key;
use crate::engine::general::objects2d::sprite::Sprite;
use engine::app::Component;
use engine::app::GameApp;
use engine::general::inputing::input::Input;
use engine::general::scene::scene::Scene;
use engine::general::time::Time;

pub struct PlayerController {
    speed: f32,
    player_sprite_index: usize,
    camera_index: usize,
}

impl PlayerController {
    pub fn new(player_sprite_index: usize, camera_index: usize) -> Self {
        Self {
            speed: 3.0,
            player_sprite_index,
            camera_index,
        }
    }
}

impl Component for PlayerController {
    fn start(&mut self) {
        println!("Player started!");
    }

    fn update(
        &mut self,
        input: &Input,
        time: &Time,
        sprites: &mut Vec<Sprite>,
        cameras: &mut Vec<Camera>,
    ) {
        let delta_time = time.get_delta_time() as f32;

        let sprite = match sprites.get_mut(self.player_sprite_index) {
            Some(s) => s,
            None => return,
        };

        let mut dx = 0.0;
        let mut dy = 0.0;
        let mut dz = 0.0;

        // Управление спрайтом (игроком)
        if input.is_key_pressed(Key::Space) {
            dy += self.speed * delta_time;
        }
        if input.is_key_pressed(Key::Shift) {
            dy -= self.speed * delta_time;
        }
        if input.is_key_pressed(Key::S) {
            dx -= self.speed * delta_time;
        }
        if input.is_key_pressed(Key::W) {
            dx += self.speed * delta_time;
        }
        if input.is_key_pressed(Key::D) {
            dz += self.speed * delta_time;
        }
        if input.is_key_pressed(Key::A) {
            dz -= self.speed * delta_time;
        }

        if dx != 0.0 || dy != 0.0 || dz != 0.0 {
            sprite.x += dx;
            sprite.y += dy;
            sprite.z += dz;
        }

        let mut cam_dx = 0.0;
        let mut cam_dy = 0.0;
        let mut cam_dz = 0.0;
        let cam_speed = 2.0;

        if input.is_key_pressed(Key::Up) {
            cam_dy += cam_speed * delta_time;
        }
        if input.is_key_pressed(Key::Down) {
            cam_dy -= cam_speed * delta_time;
        }
        if input.is_key_pressed(Key::Left) {
            cam_dx -= cam_speed * delta_time;
        }
        if input.is_key_pressed(Key::Right) {
            cam_dx += cam_speed * delta_time;
        }
        if input.is_key_pressed(Key::Control) {
            cam_dz += cam_speed * delta_time;
        }
        if input.is_key_pressed(Key::Slash) {
            cam_dz -= cam_speed * delta_time;
        }

        if cam_dx != 0.0 || cam_dy != 0.0 || cam_dz != 0.0 {
            if let Some(camera) = cameras.get_mut(self.camera_index) {
                let pos = camera.get_position();
                camera.update_position(pos.x + cam_dx, pos.y + cam_dy, pos.z + cam_dz);
            }
        }
    }
}

fn main() {
    let mut app = GameApp::new("Engine");
    let mut level_1 = Scene::new(1, "Level 1".to_string());

    let camera = Camera::new(
        (0.0, 0.0, 5.0),
        -45.0, // Yaw
        -15.0, // Pitch
        2.5,   // Speed
        0.1,   // Sensitivity
        45.0,  // Zoom/FOV
    );

    level_1.add_camera(camera);

    let player_sprite = Sprite {
        x: 0.0,
        y: 0.0,
        z: 0.0,
        width: 1.0,
        height: 1.0,
        texture_name: "rs".to_string(),
    };

    let test_sprite = Sprite {
        x: 2.0,
        y: 0.0,
        z: 0.0,
        width: 1.0,
        height: 1.0,
        texture_name: "linus".to_string(),
    };

    let p_idx = level_1.add_sprite(player_sprite);
    level_1.add_sprite(test_sprite);

    let controller = Box::new(PlayerController::new(p_idx, 0));
    level_1.add_component(controller);

    app.scene_adaptor.add_scene(level_1);
    app.run();
}
