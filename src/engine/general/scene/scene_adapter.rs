use crate::engine::general::scene::scene::Scene;
use std::collections::HashMap;

pub struct SceneAdapter {
    scenes: HashMap<usize, Scene>,
    current_scene_id: Option<usize>,
}

impl SceneAdapter {
    pub fn new() -> Self {
        Self {
            scenes: HashMap::new(),
            current_scene_id: None,
        }
    }

    pub fn add_scene(&mut self, scene: Scene) {
        let id = scene.get_id();
        self.scenes.insert(id, scene);
        if self.current_scene_id.is_none() {
            self.current_scene_id = Some(id);
        }
    }

    pub fn change_scene(&mut self, id: usize) {
        if self.scenes.contains_key(&id) {
            self.current_scene_id = Some(id);
            if let Some(scene) = self.scenes.get_mut(&id) {
                scene.start();
            }
        }
    }

    pub fn get_current_scene_mut(&mut self) -> Option<&mut Scene> {
        let id = self.current_scene_id?;
        self.scenes.get_mut(&id)
    }

    pub fn get_current_scene(&self) -> Option<&Scene> {
        let id = self.current_scene_id?;
        self.scenes.get(&id)
    }
}
