use gl::types::{GLenum, GLint, GLsizeiptr, GLuint};
use gl::{self, CreateShader};
use std::ffi::{CStr, CString, NulError};
use std::ptr;
use std::str;

pub struct Shader {
    pub id: GLuint,
}

#[derive(Debug)]
pub enum ShaderError {
    CompilationError(String),
    LinkingError(String),
    Utf8Error(std::string::FromUtf8Error),
    NulError(std::ffi::NulError),
}

impl From<std::ffi::NulError> for ShaderError {
    fn from(err: std::ffi::NulError) -> Self {
        ShaderError::NulError(err)
    }
}

impl From<std::string::FromUtf8Error> for ShaderError {
    fn from(err: std::string::FromUtf8Error) -> Self {
        ShaderError::Utf8Error(err)
    }
}

impl Shader {
    pub fn new(source_code: &str, shader_type: GLenum) -> Result<Self, ShaderError> {
        let source_code = CString::new(source_code)?;
        unsafe {
            let id = gl::CreateShader(shader_type);
            gl::ShaderSource(id, 1, &source_code.as_ptr(), ptr::null());
            gl::CompileShader(id);

            let mut success: GLint = 0;
            gl::GetShaderiv(id, gl::COMPILE_STATUS, &mut success);

            if success == gl::TRUE as GLint {
                Ok(Self { id: id as GLuint })
            } else {
                let mut error_log_size: GLint = 0;
                gl::GetShaderiv(id, gl::INFO_LOG_LENGTH, &mut error_log_size);
                let mut error_log: Vec<u8> = vec![0; error_log_size as usize];

                gl::GetShaderInfoLog(
                    id,
                    error_log_size,
                    ptr::null_mut(),
                    error_log.as_mut_ptr() as *mut _,
                );

                let log = String::from_utf8(error_log)?;
                Err(ShaderError::CompilationError(log))
            }
        }
    }
}

impl Drop for Shader {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteShader(self.id);
        }
    }
}

pub struct ShaderProgram {
    pub id: GLuint,
}

impl ShaderProgram {
    pub unsafe fn new(shaders: &[Shader]) -> Result<Self, ShaderError> {
        let program = Self {
            id: gl::CreateProgram(),
        };

        for shader in shaders {
            gl::AttachShader(program.id, shader.id);
        }

        gl::LinkProgram(program.id);

        let mut success: GLint = 0;
        gl::GetProgramiv(program.id, gl::LINK_STATUS, &mut success);

        if success == 1 {
            Ok(program)
        } else {
            let mut error_log_size: GLint = 0;
            gl::GetProgramiv(program.id, gl::INFO_LOG_LENGTH, &mut error_log_size);
            let mut error_log: Vec<u8> = Vec::with_capacity(error_log_size as usize);
            gl::GetProgramInfoLog(
                program.id,
                error_log_size,
                &mut error_log_size,
                error_log.as_mut_ptr() as *mut _,
            );

            error_log.set_len(error_log_size as usize);
            let log = String::from_utf8(error_log)?;
            Err(ShaderError::LinkingError(log))
        }
    }

    pub unsafe fn apply(&self) {
        gl::UseProgram(self.id);
    }

    pub fn get_attrib_location(&self, name: &str) -> Option<GLint> {
        unsafe {
            let c_name = CString::new(name).unwrap();
            let location = gl::GetAttribLocation(self.id, c_name.as_ptr());
            if location >= 0 { Some(location) } else { None }
        }
    }

    pub unsafe fn set_int_uniform(&self, name: &str, value: i32) -> Result<(), ShaderError> {
        self.apply();
        let uniform = CString::new(name)?;
        gl::Uniform1i(gl::GetUniformLocation(self.id, uniform.as_ptr()), value);
        Ok(())
    }
}

impl Drop for ShaderProgram {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteProgram(self.id);
        }
    }
}
