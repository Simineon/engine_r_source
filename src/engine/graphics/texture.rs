use gl::types::*;
use std::collections::HashMap;
use std::path::Path;

pub struct Texture {
    id: GLuint,
    pub width: i32,
    pub height: i32,
}

impl Texture {
    pub fn new() -> Self {
        let mut id: GLuint = 0;
        unsafe {
            gl::GenTextures(1, &mut id);
        }

        let texture = Self {
            id,
            width: 0,
            height: 0,
        };

        texture.set_wrap(gl::CLAMP_TO_EDGE as GLint, gl::CLAMP_TO_EDGE as GLint);
        texture.set_filter(gl::LINEAR as GLint, gl::LINEAR as GLint);

        texture
    }

    pub fn from_file<P: AsRef<Path>>(path: P) -> Result<Self, Box<dyn std::error::Error>> {
        let mut texture = Self::new();
        texture.load(path)?;
        Ok(texture)
    }

    pub fn bind(&self) {
        unsafe {
            gl::BindTexture(gl::TEXTURE_2D, self.id);
        }
    }

    pub fn unbind(&self) {
        unsafe {
            gl::BindTexture(gl::TEXTURE_2D, 0);
        }
    }

    pub fn set_image(&mut self, width: i32, height: i32, data: &[u8]) {
        self.width = width;
        self.height = height;
        self.bind();
        unsafe {
            gl::TexImage2D(
                gl::TEXTURE_2D,
                0,
                gl::RGBA8 as i32,
                width,
                height,
                0,
                gl::RGBA,
                gl::UNSIGNED_BYTE,
                data.as_ptr() as *const _,
            );
        }
    }

    pub fn load<P: AsRef<Path>>(&mut self, path: P) -> Result<(), Box<dyn std::error::Error>> {
        let img = image::open(path)?.into_rgba8();
        self.set_image(img.width() as i32, img.height() as i32, img.as_raw());
        Ok(())
    }

    pub fn set_filter(&self, min: GLint, mag: GLint) {
        self.bind();
        unsafe {
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, min);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, mag);
        }
    }

    pub fn set_wrap(&self, s: GLint, t: GLint) {
        self.bind();
        unsafe {
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_S, s);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_T, t);
        }
    }

    pub fn get_param(&self, param: GLenum) -> GLint {
        let mut value: GLint = 0;
        self.bind();
        unsafe {
            gl::GetTexLevelParameteriv(gl::TEXTURE_2D, 0, param, &mut value);
        }
        value
    }

    pub fn get_width(&self) -> i32 {
        self.get_param(gl::TEXTURE_WIDTH)
    }

    pub fn get_height(&self) -> i32 {
        self.get_param(gl::TEXTURE_HEIGHT)
    }

    pub fn activate(&self, unit: u32) {
        unsafe {
            gl::ActiveTexture(gl::TEXTURE0 + unit);
            gl::BindTexture(gl::TEXTURE_2D, self.id);
        }
    }
}

impl Drop for Texture {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteTextures(1, &self.id);
        }
    }
}

pub fn load_all_textures_from_assets() -> (Vec<Texture>, HashMap<String, u32>) {
    let mut textures = Vec::new();
    let mut texture_registry = HashMap::new();

    let extensions = ["png", "jpg", "jpeg", "tga", "webp"];
    let assets_dir = Path::new("assets");

    if let Ok(entries) = std::fs::read_dir(assets_dir) {
        for entry in entries.flatten() {
            let path = entry.path();

            if path.is_file() {
                if let Some(ext) = path.extension().and_then(|s| s.to_str()) {
                    if extensions.contains(&ext.to_lowercase().as_str()) {
                        if let Some(file_name) = path.file_name().and_then(|s| s.to_str()) {
                            if textures.len() >= 16 {
                                eprintln!(
                                    "Warning: Maximum 16 textures supported, skipping: {}",
                                    file_name
                                );
                                continue;
                            }

                            let mut texture = Texture::new();
                            if let Ok(_) = texture.load(&path) {
                                let index = textures.len() as u32;
                                textures.push(texture);

                                let name_without_ext = path
                                    .file_stem()
                                    .and_then(|s| s.to_str())
                                    .unwrap_or(file_name)
                                    .to_string();
                                texture_registry.insert(name_without_ext, index);

                                println!(
                                    "Texture loaded successfully [slot {}]: {}",
                                    index, file_name
                                );
                            }
                        }
                    }
                }
            }
        }
    } else {
        eprintln!("Error: Folder 'assets' was NOT found in project!");
    }

    (textures, texture_registry)
}
