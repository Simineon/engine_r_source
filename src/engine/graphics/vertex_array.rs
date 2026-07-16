use gl::types::{GLenum, GLint, GLsizeiptr, GLuint};
use gl::{self, CreateShader};
use std::ffi::{CStr, CString, NulError};

pub struct VertexArray {
    pub id: GLuint,
}

impl VertexArray {
    pub unsafe fn new() -> Self {
        let mut id: GLuint = 0;
        gl::GenVertexArrays(1, &mut id);
        Self { id }
    }
}

impl Drop for VertexArray {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteVertexArrays(1, [self.id].as_ptr());
        }
    }
}

impl VertexArray {
    pub unsafe fn bind(&self) {
        gl::BindVertexArray(self.id);
    }
}

impl VertexArray {
    pub unsafe fn set_attribute<V: Sized>(
        &self,
        attrib_pos: GLint,
        components: GLint,
        offset: GLint,
    ) {
        if attrib_pos < 0 {
            return;
        }

        self.bind();
        gl::VertexAttribPointer(
            attrib_pos as GLuint,
            components,
            gl::FLOAT,
            gl::FALSE,
            std::mem::size_of::<V>() as GLint,
            offset as *const _,
        );
        gl::EnableVertexAttribArray(attrib_pos as GLuint);
    }
}
