use crate::engine::general::entity::entity::{Entity, get_all_game_objects, register_game_object};

pub struct EntityHierarchy {
    pub is_parent: Option<bool>,
    pub is_children: Option<bool>,
    pub entities: Vec<Entity>,
}

impl EntityHierarchy {
    pub fn new() -> Self {
        Self {
            is_parent: None,
            is_children: None,
            entities: Vec::new(),
        }
    }

    pub fn add_entity(&mut self, entity: Entity) {
        register_game_object(entity);
        self.entities.push(entity);
    }

    pub fn get_game_objects(&self) -> &Vec<Entity> {
        &self.entities
    }

    pub fn get_game_objects_str(&self) -> String {
        if self.entities.is_empty() {
            return "No game objects".to_string();
        }

        self.entities
            .iter()
            .map(|entity| format!("Entity with id {}", entity.0))
            .collect::<Vec<String>>()
            .join(", ")
    }

    pub fn refresh_game_objects(&mut self) {
        self.entities = get_all_game_objects();
    }

    pub fn is_game_object(&self, entity: Entity) -> bool {
        self.entities.contains(&entity)
    }
}
