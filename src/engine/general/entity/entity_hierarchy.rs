use crate::engine::general::entity::entity::{Entity, get_all_game_objects, register_game_object};

pub struct EntityHierarchy {
    pub parent: Option<Vec<Entity>>,
    pub children: Option<Vec<Entity>>,
    pub entities: Vec<Entity>,
    pub game_objects: Vec<Entity>,
}

impl EntityHierarchy {
    pub fn new() -> Self {
        Self {
            parent: None,
            children: None,
            entities: Vec::new(),
            game_objects: Vec::new(),
        }
    }

    pub fn add_entity(&mut self, entity: Entity, register: bool) {
        self.entities.push(entity);

        if register {
            register_game_object(entity);
            self.game_objects.push(entity);
        }
    }

    pub fn add_game_object(&mut self, entity: Entity) {
        self.entities.push(entity);
        register_game_object(entity);
        self.game_objects.push(entity);
    }

    pub fn get_game_objects(&self) -> &Vec<Entity> {
        &self.game_objects
    }

    pub fn get_game_objects_str(&self) -> String {
        if self.game_objects.is_empty() {
            return "No game objects".to_string();
        }

        self.game_objects
            .iter()
            .map(|entity| format!("Entity with id: {}", entity.0))
            .collect::<Vec<String>>()
            .join(", ")
    }

    pub fn refresh_game_objects(&mut self) {
        self.game_objects = get_all_game_objects();
    }

    pub fn is_game_object(&self, entity: Entity) -> bool {
        self.game_objects.contains(&entity)
    }
}
