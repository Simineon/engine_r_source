use crate::engine::app::Component;
use crate::engine::general::camera::Camera;
use crate::engine::general::coordination::{Coords, Location};
use crate::engine::general::entity::entity::Entity;
use crate::engine::general::inputing::input::Input;
use crate::engine::general::objects2d::sprite::{Sprite, sprite_group};
use crate::engine::general::time::Time;
use std::collections::HashMap;

pub struct Scene {
    id: usize,
    name: String,
    pub entities: Vec<Entity>,
    components: Vec<Box<dyn Component>>,
    pub cameras: Vec<Camera>,
    entity_sprite_map: HashMap<Entity, usize>,
    global_coords: Coords,
}

impl Scene {
    pub fn new(id: usize, name: String) -> Self {
        Self {
            id,
            name,
            entities: Vec::new(),
            components: Vec::new(),
            cameras: Vec::new(),
            entity_sprite_map: HashMap::new(),
            global_coords: Coords {
                x: 0.0,
                y: 0.0,
                z: 0.0,
            },
        }
    }

    pub fn update_camera_to_follow_sprite(&mut self, entity: Entity, camera_index: usize) {
        if let Some(&sprite_index) = self.entity_sprite_map.get(&entity) {
            let sprites = sprite_group.lock().unwrap();
            if let Some(sprite) = sprites.get(sprite_index) {
                if let Some(camera) = self.cameras.get_mut(camera_index) {
                    camera.update_position(
                        sprite.location.position.x,
                        sprite.location.position.y,
                        sprite.location.position.z,
                    );
                }
            }
        }
    }

    pub fn add_sprite(&mut self, sprite: Sprite) -> Entity {
        let entity = sprite.get_entity();
        self.entities.push(entity);

        let sprites = sprite_group.lock().unwrap();
        if let Some(index) = sprites.iter().position(|s| s.entity == entity) {
            self.entity_sprite_map.insert(entity, index);
        }

        entity
    }

    pub fn get_sprite(&self, entity: Entity) -> Option<Sprite> {
        let sprites = sprite_group.lock().unwrap();
        if let Some(&sprite_index) = self.entity_sprite_map.get(&entity) {
            return sprites.get(sprite_index).cloned();
        }
        None
    }

    pub fn update_sprite_position(&mut self, entity: Entity, x: f32, y: f32, z: f32) -> bool {
        let mut sprites = sprite_group.lock().unwrap();
        if let Some(&sprite_index) = self.entity_sprite_map.get(&entity) {
            if let Some(sprite) = sprites.get_mut(sprite_index) {
                sprite.location.position.x = x;
                sprite.location.position.y = y;
                sprite.location.position.z = z;
                return true;
            }
        }
        false
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
        let mut all_sprites = sprite_group.lock().unwrap();

        let mut scene_sprites: Vec<Sprite> = Vec::new();
        for &entity in &self.entities {
            if let Some(&sprite_index) = self.entity_sprite_map.get(&entity) {
                if let Some(sprite) = all_sprites.get(sprite_index) {
                    scene_sprites.push(sprite.clone());
                }
            }
        }

        for component in &mut self.components {
            component.update(input, time, &mut scene_sprites, &mut self.cameras);
        }

        for (i, sprite) in scene_sprites.iter().enumerate() {
            if let Some(&entity) = self.entities.get(i) {
                if let Some(&sprite_index) = self.entity_sprite_map.get(&entity) {
                    if let Some(global_sprite) = all_sprites.get_mut(sprite_index) {
                        global_sprite.location.position.x = sprite.location.position.x;
                        global_sprite.location.position.y = sprite.location.position.y;
                        global_sprite.location.position.z = sprite.location.position.z;
                    }
                }
            }
        }
    }

    pub fn get_id(&self) -> usize {
        self.id
    }

    pub fn get_name(&self) -> &str {
        &self.name
    }

    pub fn get_entities(&self) -> &Vec<Entity> {
        &self.entities
    }
}
