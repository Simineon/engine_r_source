//! Renderer file
//!
//! This file setting shader program
use crate::engine::general::scene::scene::Scene;
use crate::engine::graphics::mesh::mesh::Mesh;
use crate::engine::graphics::shader::{Shader, ShaderProgram};
use crate::engine::graphics::texture::Texture;
use crate::engine::graphics::vertex::Vertex;
use std::collections::HashMap;

pub struct Renderer {
    program: ShaderProgram,
    textures: Vec<Texture>,
}

impl Renderer {
    pub fn new() -> Self {
        // set shader's resources
        let program = unsafe {
            let vs_src = std::fs::read_to_string("static/shader.vert").expect("VS missing");
            let fs_src = std::fs::read_to_string("static/shader.frag").expect("FS missing");
            let vs = Shader::new(&vs_src, gl::VERTEX_SHADER).expect("VS Error");
            let fs = Shader::new(&fs_src, gl::FRAGMENT_SHADER).expect("FS Error");
            ShaderProgram::new(&[vs, fs]).expect("Program Error")
        };

        // init
        unsafe {
            program.apply();
        }

        Self {
            program,
            textures: Vec::new(),
        }
    }

    pub fn draw(
        &self,
        scene: &Scene,
        mesh: &mut Mesh,
        textures: &[Texture],
        texture_registry: &HashMap<String, u32>,
    ) {
        unsafe {
            gl::ClearColor(0.2, 0.3, 0.3, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT);

            self.program.apply();

            // get vertices
            let mut dynamic_vertices: Vec<Vertex> = Vec::new();
            let mut dynamic_indices: Vec<u32> = Vec::new();
            let mut used_textures = std::collections::HashSet::new();

            for sprite in scene.sprites.iter() {
                sprite.append_vertices(
                    &mut dynamic_vertices,
                    &mut dynamic_indices,
                    texture_registry,
                );

                if let Some(&index) = texture_registry.get(&sprite.texture_name) {
                    used_textures.insert(index);
                }
            }

            // update
            mesh.update_vertices(&dynamic_vertices);
            mesh.update_indices(&dynamic_indices);

            // Activating textures
            for &texture_index in used_textures.iter() {
                if texture_index < textures.len() as u32 {
                    textures[texture_index as usize].activate(texture_index as u32);
                }
            }

            mesh.draw();
        }
    }

    pub fn get_shader_program(&self) -> &ShaderProgram {
        // returning link, don't cloning because shader program general
        &self.program
    }
}
