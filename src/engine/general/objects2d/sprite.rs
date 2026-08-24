use crate::engine::general::entity::entity::{Entity, get_all_game_objects, register_game_object};
use crate::engine::graphics::vertex::Vertex;
use lazy_static::lazy_static;
use std::collections::HashMap;
use std::sync::Mutex;

lazy_static! {
    pub static ref sprite_group: Mutex<Vec<Sprite>> = Mutex::new(Vec::new());
}

#[derive(Debug)]
pub struct Sprite {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub width: f32,
    pub height: f32,
    pub texture_name: String,
    pub entity: Entity,
    pub is_game_object: bool,
}

impl Sprite {
    pub fn new(x: f32, y: f32, z: f32, width: f32, height: f32, texture_name: &str) -> Self {
        Self {
            x,
            y,
            z,
            width,
            height,
            texture_name: texture_name.to_string(),
            entity: Entity::new(),
            is_game_object: false,
        }
    }

    pub fn new_game_object(
        x: f32,
        y: f32,
        z: f32,
        width: f32,
        height: f32,
        texture_name: &str,
    ) -> Self {
        let mut sprite = Self::new(x, y, z, width, height, texture_name);
        sprite.register_as_game_object();
        sprite
    }

    pub fn register_as_game_object(&mut self) {
        if !self.is_game_object {
            register_game_object(self.entity);
            self.is_game_object = true;
            println!(
                "Sprite registered as game object: Entity #{}",
                self.entity.0
            );
        }
    }

    pub fn append_vertices(
        &self,
        vertices: &mut Vec<Vertex>,
        indices: &mut Vec<u32>,
        registry: &HashMap<String, u32>,
    ) {
        let base_index = vertices.len() as u32;

        let tex_id = *registry.get(&self.texture_name).unwrap_or(&0) as f32;

        let x0 = self.x - self.width / 2.0;
        let x1 = self.x + self.width / 2.0;
        let y0 = self.y - self.height / 2.0;
        let y1 = self.y + self.height / 2.0;
        let z = self.z;

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
        self.x = new_x;
        self.y = new_y;
    }
}

impl Clone for Sprite {
    fn clone(&self) -> Self {
        Self {
            x: self.x,
            y: self.y,
            z: self.z,
            width: self.width,
            height: self.height,
            texture_name: self.texture_name.clone(),
            entity: self.entity,
            is_game_object: false,
        }
    }
}

#[macro_export]
macro_rules! sprite_game_object {
    ($x:expr, $y:expr, $z:expr, $width:expr, $height:expr, $texture:expr) => {{
        let mut sprite = Sprite::new($x, $y, $z, $width, $height, $texture);
        sprite.register_as_game_object();
        sprite
    }};
}
