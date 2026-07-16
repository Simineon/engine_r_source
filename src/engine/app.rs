//! Here's main loop and another main thing...
//! Sorry Richy! I forgot end 3d(I can't do loading models and render in one day), end bugs of architecture. I wanted to make
//! one other thing in engine...
//! But I'm yet don't will yet.
//! I'm need to go to my grandma
//! I'm going to village(tomorrow)
//!
//!
//! When I will come back to city I'll will update this repo.
//! Well, now engine have something to show(Sprites and Scenes for example)
//! You may see componets - this is my implementation of scripts system from popular game engines(unity for example: MonoBehaviur Script)
//! For FS!
//! Hiel Linus!

use crate::engine::general::inputing::input::Input;
use crate::engine::general::inputing::keys::Key;
use crate::engine::general::objects2d::sprite::Sprite;
use crate::engine::general::scene::scene_adapter::SceneAdapter;
use crate::engine::general::time::Time;
use crate::engine::general::window::Window;
use crate::engine::graphics::buffer::Buffer;
use crate::engine::graphics::shader::{Shader, ShaderProgram};
use crate::engine::graphics::texture::{Texture, load_all_textures_from_assets};
use crate::engine::graphics::vertex::*;
use crate::engine::graphics::vertex_array::VertexArray;
use glutin::event::{Event, WindowEvent};
use glutin::event_loop::{ControlFlow, EventLoop};
use std::ptr;

pub trait Component {
    fn start(&mut self);
    fn update(&mut self, input: &Input, time: &Time, sprites: &mut Vec<Sprite>);
}

pub struct GameApp {
    name: String,
    time: Time,
    input: Input,
    pub scene_adaptor: SceneAdapter,
}

impl GameApp {
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            time: Time::new(),
            input: Input::new(),
            scene_adaptor: SceneAdapter::new(),
        }
    }

    pub fn run(mut self) {
        let event_loop = EventLoop::new();
        let window = Window::new(&self.name, &event_loop);
        let gl_context = window.gl_context;

        let (mut textures, mut texture_registry) = load_all_textures_from_assets();

        if textures.is_empty() {
            let mut dummy_texture = Texture::new();
            let dummy_data = vec![255, 0, 255, 255];
            dummy_texture.set_image(1, 1, &dummy_data);
            textures.push(dummy_texture);
            texture_registry.insert("default".to_string(), 0);
            println!("Created default texture for missing textures");
        }

        let program = unsafe {
            let vs_src = std::fs::read_to_string("static/shader.vert").expect("VS missing");
            let fs_src = std::fs::read_to_string("static/shader.frag").expect("FS missing");
            let vs = Shader::new(&vs_src, gl::VERTEX_SHADER).expect("VS Error");
            let fs = Shader::new(&fs_src, gl::FRAGMENT_SHADER).expect("FS Error");
            ShaderProgram::new(&[vs, fs]).expect("Program Error")
        };

        unsafe {
            program.apply();
            for i in 0..textures.len() {
                let name = format!("u_Textures[{}]", i);
                let _ = program.set_int_uniform(&name, i as i32);
            }
        }

        let mut dynamic_vertices: Vec<Vertex> = Vec::new();
        let mut dynamic_indices: Vec<u32> = Vec::new();

        if let Some(scene) = self.scene_adaptor.get_current_scene() {
            for sprite in scene.sprites.iter() {
                sprite.append_vertices(
                    &mut dynamic_vertices,
                    &mut dynamic_indices,
                    &texture_registry,
                );
            }
        }

        let (vbo, vao, ebo) = unsafe {
            let vao = VertexArray::new();
            let vbo = Buffer::new(gl::ARRAY_BUFFER);
            let ebo = Buffer::new(gl::ELEMENT_ARRAY_BUFFER);

            vao.bind();
            vbo.bind();
            vbo.set_data(&dynamic_vertices, gl::DYNAMIC_DRAW);

            ebo.bind();
            ebo.set_data(&dynamic_indices, gl::DYNAMIC_DRAW);

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

        if let Some(scene) = self.scene_adaptor.get_current_scene_mut() {
            scene.start();
        }

        event_loop.run(move |event, _, control_flow| {
            *control_flow = ControlFlow::Poll;
            let _fetch_resources = (&vao, &vbo, &ebo, &program);

            match event {
                Event::WindowEvent { event, .. } => match event {
                    WindowEvent::CloseRequested => *control_flow = ControlFlow::Exit,
                    WindowEvent::Resized(size) => gl_context.resize(size),
                    WindowEvent::KeyboardInput { input, .. } => {
                        if let Some(virtual_keycode) = input.virtual_keycode {
                            let key = Key::from(virtual_keycode);
                            match input.state {
                                glutin::event::ElementState::Pressed => {
                                    self.input.on_key_press(key);
                                }
                                glutin::event::ElementState::Released => {
                                    self.input.on_key_release(key);
                                }
                            }
                        }
                    }
                    _ => (),
                },
                Event::MainEventsCleared => {
                    self.time.update_delta_time();
                    let dt = self.time.get_delta_time();

                    self.input.update();

                    println!("FPS: {:.2}", 1.0 / dt);

                    if let Some(scene) = self.scene_adaptor.get_current_scene_mut() {
                        scene.update(&self.input, &self.time);
                    }
                    gl_context.window().request_redraw();
                }
                Event::RedrawRequested(_) => {
                    dynamic_vertices.clear();
                    dynamic_indices.clear();

                    let mut used_textures = std::collections::HashSet::new();

                    if let Some(scene) = self.scene_adaptor.get_current_scene() {
                        for sprite in scene.sprites.iter() {
                            sprite.append_vertices(
                                &mut dynamic_vertices,
                                &mut dynamic_indices,
                                &texture_registry,
                            );

                            if let Some(&index) = texture_registry.get(&sprite.texture_name) {
                                used_textures.insert(index);
                            }
                        }
                    }

                    unsafe {
                        gl::ClearColor(0.2, 0.3, 0.3, 1.0);
                        gl::Clear(gl::COLOR_BUFFER_BIT);

                        program.apply();

                        vbo.bind();
                        vbo.set_data(&dynamic_vertices, gl::DYNAMIC_DRAW);

                        ebo.bind();
                        ebo.set_data(&dynamic_indices, gl::DYNAMIC_DRAW);

                        for &texture_index in used_textures.iter() {
                            if texture_index < textures.len() as u32 {
                                textures[texture_index as usize].activate(texture_index as u32);
                            }
                        }

                        vao.bind();
                        gl::DrawElements(
                            gl::TRIANGLES,
                            dynamic_indices.len() as i32,
                            gl::UNSIGNED_INT,
                            ptr::null(),
                        );
                    }
                    gl_context.swap_buffers().unwrap();
                }
                _ => (),
            }
        });
    }
}
