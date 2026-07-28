use crate::engine::app::Component;
use crate::engine::general::camera::Camera;
use crate::engine::general::inputing::input::Input;
use crate::engine::general::objects2d::sprite::Sprite;
use crate::engine::general::time::Time;

pub struct Scene {
    id: usize,
    name: String,
    pub sprites: Vec<Sprite>,
    components: Vec<Box<dyn Component>>,
    pub cameras: Vec<Camera>,
}

impl Scene {
    pub fn new(id: usize, name: String) -> Self {
        Self {
            id,
            name,
            sprites: Vec::new(),
            components: Vec::new(),
            cameras: Vec::new(),
        }
    }

    pub fn update_camera_to_follow_sprite(&mut self, sprite_index: usize, camera_index: usize) {
        if let Some(sprite) = self.sprites.get(sprite_index) {
            if let Some(camera) = self.cameras.get_mut(camera_index) {
                camera.update_position(sprite.x, sprite.y, sprite.z);
            }
        }
    }

    pub fn add_sprite(&mut self, sprite: Sprite) -> usize {
        self.sprites.push(sprite);
        self.sprites.len() - 1 // Return sprite's index in this Scene
    }

    pub fn add_component(&mut self, component: Box<dyn Component>) {
        self.components.push(component);
    }

    pub fn add_camera(&mut self, camera: Camera) {
        self.cameras.push(camera);
    }

    pub fn start(&mut self) {
        for comp in self.components.iter_mut() {
            comp.start();
        }
    }

    pub fn update(&mut self, input: &Input, time: &Time) {
        let sprites = &mut self.sprites;
        let cameras = &mut self.cameras;

        for component in &mut self.components {
            component.update(input, time, sprites, cameras);
        }
    }

    pub fn get_id(&self) -> usize {
        self.id
    }
    pub fn get_name(&self) -> &str {
        &self.name
    }
}
