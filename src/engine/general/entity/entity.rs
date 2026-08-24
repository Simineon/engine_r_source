use lazy_static::lazy_static;
use std::sync::Mutex;

lazy_static! {
    static ref ENTITY_IDS: Mutex<Vec<u64>> = Mutex::new(Vec::new());
    static ref NEXT_ENTITY_ID: Mutex<u64> = Mutex::new(1);
    pub static ref GAME_OBJECTS: Mutex<Vec<Entity>> = Mutex::new(Vec::new());
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Entity(pub u64);

impl Entity {
    pub fn new() -> Self {
        let mut next_id = NEXT_ENTITY_ID.lock().unwrap();
        let id = *next_id;
        *next_id += 1;

        let mut ids = ENTITY_IDS.lock().unwrap();
        ids.push(id);

        println!("New entity was created with id: {}", &id);
        Entity(id)
    }
}

impl std::fmt::Display for Entity {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

pub fn get_all_entity_ids() -> Vec<u64> {
    ENTITY_IDS.lock().unwrap().clone()
}

pub fn get_all_game_objects() -> Vec<Entity> {
    GAME_OBJECTS.lock().unwrap().clone()
}

pub fn register_game_object(entity: Entity) {
    let mut objects = GAME_OBJECTS.lock().unwrap();
    if !objects.contains(&entity) {
        objects.push(entity);
        println!("Game object registered: Entity {}", entity.0);
    }
}

pub fn is_game_object(entity: Entity) -> bool {
    let objects = GAME_OBJECTS.lock().unwrap();
    objects.contains(&entity)
}

#[derive(Debug, Clone)]
pub struct Location {
    pub position: [f32; 3],
    pub rotation: [f32; 4],
    pub scale: [f32; 3],
}

#[macro_export]
macro_rules! register_game_object {
    ($instance:expr) => {{
        let entity = $instance.get_entity();
        $crate::engine::general::entity::entity::register_game_object(entity);
        $instance
    }};
}
