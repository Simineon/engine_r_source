use crate::engine::app::Component;
use crate::engine::general::inputing::input::Input;
use crate::engine::general::objects2d::sprite::Sprite;
use crate::engine::general::time::Time;
use std::collections::HashMap;

pub struct Scene {
    id: usize,
    name: String,
    pub sprites: Vec<Sprite>,
    components: Vec<Box<dyn Component>>,
}

impl Scene {
    pub fn new(id: usize, name: String) -> Self {
        Self {
            id,
            name,
            sprites: Vec::new(),
            components: Vec::new(),
        }
    }

    pub fn add_sprite(&mut self, sprite: Sprite) -> usize {
        self.sprites.push(sprite);
        self.sprites.len() - 1 // Return sprite's index in this Scene
    }

    pub fn add_component(&mut self, component: Box<dyn Component>) {
        self.components.push(component);
    }

    pub fn start(&mut self) {
        for comp in self.components.iter_mut() {
            comp.start();
        }
    }

    pub fn update(&mut self, input: &Input, time: &Time) {
        for comp in self.components.iter_mut() {
            comp.update(input, time, &mut self.sprites);
        }
    }

    pub fn get_id(&self) -> usize {
        self.id
    }
    pub fn get_name(&self) -> &str {
        &self.name
    }
}
