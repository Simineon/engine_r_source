use crate::engine::graphics::vertex::Vertex;
use lazy_static::lazy_static;
use std::collections::HashMap;
use std::sync::Mutex;

lazy_static! {
    pub static ref sprite_group: Mutex<Vec<Sprite>> = Mutex::new(Vec::new());
}

pub struct Sprite {
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub height: f32,
    pub texture_name: String,
}

impl Sprite {
    pub fn new(x: f32, y: f32, width: f32, height: f32, texture_name: &str) {
        sprite_group.lock().unwrap().push(Self {
            x,
            y,
            width,
            height,
            texture_name: texture_name.to_string(),
        });
    }

    pub fn append_vertices(
        &self,
        vertices: &mut Vec<Vertex>,
        indices: &mut Vec<u32>,
        registry: &HashMap<String, u32>,
    ) {
        let base_index = vertices.len() as u32;

        let tex_id = *registry.get(&self.texture_name).unwrap_or(&0) as f32;

        let x0 = self.x;
        let x1 = self.x + self.width;
        let y0 = self.y;
        let y1 = self.y + self.height;

        vertices.push(Vertex([x0, y0], [0.0, 1.0], tex_id)); // Bottom-left
        vertices.push(Vertex([x1, y0], [1.0, 1.0], tex_id)); // Bottom-right
        vertices.push(Vertex([x1, y1], [1.0, 0.0], tex_id)); // Top-right
        vertices.push(Vertex([x0, y1], [0.0, 0.0], tex_id)); // Top-left

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
        // Семён, запомни &mut self с mut потому что, только так self-объекты становятся изменяемыми, иначе компилятор шлёт нахуй
        self.x = new_x;
        self.y = new_y;
    }
}
