pub type Pos = [f32; 2];
pub type TextureCoords = [f32; 2];

#[repr(C)]
pub struct Vertex(pub Pos, pub TextureCoords, pub f32);
