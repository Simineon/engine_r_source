use crate::engine::graphics::shader::ShaderProgram;
use crate::engine::graphics::{
    buffer::Buffer, vertex::Pos, vertex::TextureCoords, vertex::Vertex, vertex_array::VertexArray,
};
use gl;

pub struct Mesh {
    vao: VertexArray,
    vbo: Buffer,
    ebo: Buffer,
    vertex_count: i64,
}

impl Mesh {
    pub fn new(vertices: &[Vertex], indices: &[u32], program: &ShaderProgram) -> Self {
        let (vbo, vao, ebo) = unsafe {
            let vao = VertexArray::new();
            let vbo = Buffer::new(gl::ARRAY_BUFFER);
            let ebo = Buffer::new(gl::ELEMENT_ARRAY_BUFFER);

            vao.bind();
            vbo.bind();
            vbo.set_data(vertices, gl::DYNAMIC_DRAW);

            ebo.bind();
            ebo.set_data(indices, gl::DYNAMIC_DRAW);

            // using transfered shader program
            let pos_attrib = program.get_attrib_location("position").unwrap_or(0);
            let tex_attrib = program.get_attrib_location("texCoord").unwrap_or(1);
            let index_attrib = program.get_attrib_location("aTexIndex").unwrap_or(2);

            let tex_offset = std::mem::size_of::<Pos>() as i32;
            let index_offset =
                (std::mem::size_of::<Pos>() + std::mem::size_of::<TextureCoords>()) as i32;

            vao.set_attribute::<Vertex>(pos_attrib, 2, 0);
            vao.set_attribute::<Vertex>(tex_attrib, 2, tex_offset);
            vao.set_attribute::<Vertex>(index_attrib, 1, index_offset);

            (vbo, vao, ebo)
        };

        Self {
            vao,
            vbo,
            ebo,
            vertex_count: vertices.len() as i64,
        }
    }

    pub fn update_vertices(&self, vertices: &[Vertex]) {
        unsafe {
            self.vbo.bind();
            self.vbo.set_data(vertices, gl::DYNAMIC_DRAW);
        }
    }

    pub fn update_indices(&mut self, indices: &[u32]) {
        unsafe {
            self.ebo.bind();
            self.ebo.set_data(indices, gl::DYNAMIC_DRAW);
        }
        self.vertex_count = indices.len() as i64;
    }

    pub fn draw(&self) {
        unsafe {
            self.vao.bind();
            // Drawing
            gl::DrawElements(
                gl::TRIANGLES,
                self.vertex_count as i32,
                gl::UNSIGNED_INT,
                std::ptr::null(),
            );
        }
    }
}
