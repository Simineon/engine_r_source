//! The Coordinating in game world(scene)
//! - must be adapted in Scene structure
// Local coordinates are local relative to the scene's global coordinates
#[derive(Debug, Clone, PartialEq)]
pub struct Coords {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Default for Coords {
    fn default() -> Self {
        Self {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        }
    }
}

// every game object must to have Location
#[derive(Debug, Clone, PartialEq)]
pub struct Location {
    pub position: Coords,
    pub local_position: Option<Coords>,
    pub rotation: [f32; 4],
    pub scale: [f32; 3],
}

impl Default for Location {
    fn default() -> Self {
        Self {
            position: Coords::default(),
            local_position: None,
            rotation: [0.0, 0.0, 0.0, 1.0], // identity quat
            scale: [1.0, 1.0, 1.0],         // camera just ignores it
        }
    }
}
