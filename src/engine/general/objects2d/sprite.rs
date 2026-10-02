use crate::engine::general::coordination::{Coords, Location};
use crate::engine::general::entity::entity::{Entity, register_game_object};
use crate::engine::graphics::vertex::Vertex;
use lazy_static::lazy_static;
use std::collections::HashMap;
use std::sync::Mutex;

lazy_static! {
    pub static ref sprite_group: Mutex<Vec<Sprite>> = Mutex::new(Vec::new());
    pub static ref SPRITE_REGISTRY: Mutex<HashMap<Entity, usize>> = Mutex::new(HashMap::new());
}

#[derive(Debug)]
pub struct Sprite {
    // location --
    pub location: Location,
    // --
    pub width: f32,
    pub height: f32,
    pub texture_name: String,
    pub entity: Entity,
}

impl Sprite {
    pub fn new(x: f32, y: f32, z: f32, width: f32, height: f32, texture_name: &str) -> Self {
        let entity = Entity::new();
        register_game_object(entity);

        let local_coords = Coords { x, y, z };

        let sprite = Self {
            location: Location {
                position: local_coords,
                local_position: None,
                rotation: [0.0, 0.0, 0.0, 1.0],
                scale: [1.0, 1.0, 1.0],
            },
            width,
            height,
            texture_name: texture_name.to_string(),
            entity,
        };

        let mut sprites = sprite_group.lock().unwrap();
        let index = sprites.len();
        sprites.push(sprite.clone());

        let mut registry = SPRITE_REGISTRY.lock().unwrap();
        registry.insert(entity, index);

        sprite
    }

    pub fn append_vertices(
        &self,
        vertices: &mut Vec<Vertex>,
        indices: &mut Vec<u32>,
        registry: &HashMap<String, u32>,
    ) {
        let base_index = vertices.len() as u32;

        let tex_id = *registry.get(&self.texture_name).unwrap_or(&0) as f32;

        let x0 = self.location.position.x - self.width / 2.0;
        let x1 = self.location.position.x + self.width / 2.0;
        let y0 = self.location.position.y - self.height / 2.0;
        let y1 = self.location.position.y + self.height / 2.0;
        let z = self.location.position.z;

        vertices.push(Vertex([x0, y0, z], [0.0, 1.0], tex_id));
        vertices.push(Vertex([x1, y0, z], [1.0, 1.0], tex_id));
        vertices.push(Vertex([x1, y1, z], [1.0, 0.0], tex_id));
        vertices.push(Vertex([x0, y1, z], [0.0, 0.0], tex_id));

        indices.extend_from_slice(&[
            base_index + 0,
            base_index + 1,
            base_index + 2,
            base_index + 2,
            base_index + 3,
            base_index + 0,
        ]);
    }

    pub fn set_coords(&mut self, new_x: f32, new_y: f32) {
        self.location.position.x = new_x;
        self.location.position.y = new_y;
    }

    pub fn get_entity(&self) -> Entity {
        self.entity
    }
}

pub fn get_sprite_by_entity(entity: Entity) -> Option<Sprite> {
    let registry = SPRITE_REGISTRY.lock().unwrap();
    if let Some(&index) = registry.get(&entity) {
        let sprites = sprite_group.lock().unwrap();
        return sprites.get(index).cloned();
    }
    None
}

impl Clone for Sprite {
    fn clone(&self) -> Self {
        Self {
            location: self.location.clone(),
            width: self.width,
            height: self.height,
            texture_name: self.texture_name.clone(),
            entity: self.entity,
        }
    }
}
