//! Here's main loop and another main thing...
//! Sorry Richy! I forgot end 3d(I can't do loading models and render in one day), end bugs of architecture. I wanted to make
//! one other thing in engine...
//!
//! Well, now engine have something to show(Sprites and Scenes for example)
//! You may see componets - this is my implementation of scripts system from popular game engines(unity for example: MonoBehaviur Script)
//! For FS!
//! Heil Linus!
use crate::engine::general::inputing::input::Input;
use crate::engine::general::inputing::keys::Key;
use crate::engine::general::objects2d::sprite::Sprite;
use crate::engine::general::scene::scene_adapter::SceneAdapter;
use crate::engine::general::time::Time;
use crate::engine::general::window::Window;
use crate::engine::graphics::buffer::Buffer;
use crate::engine::graphics::mesh::mesh::Mesh;
use crate::engine::graphics::renderer::Renderer;
use crate::engine::graphics::texture::{Texture, load_all_textures_from_assets};
use crate::engine::graphics::vertex::*;
use crate::engine::graphics::vertex_array::VertexArray;
use glutin::event::{Event, WindowEvent};
use glutin::event_loop::{ControlFlow, EventLoop};

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

        let renderer = Renderer::new();

        let (mut textures, mut texture_registry) = load_all_textures_from_assets();

        if textures.is_empty() {
            let mut dummy_texture = Texture::new();
            let dummy_data = vec![255, 0, 255, 255];
            dummy_texture.set_image(1, 1, &dummy_data);
            textures.push(dummy_texture);
            texture_registry.insert("default".to_string(), 0);
            println!("Created default texture for missing textures");
        }

        let program = renderer.get_shader_program();

        let vertices: Vec<Vertex> = vec![
            Vertex([-0.5, -0.5], [0.0, 0.0], 0.0),
            Vertex([0.5, -0.5], [1.0, 0.0], 0.0),
            Vertex([0.5, 0.5], [1.0, 1.0], 0.0),
            Vertex([-0.5, 0.5], [0.0, 1.0], 0.0),
        ];

        let indices: Vec<u32> = vec![0, 1, 2, 0, 2, 3];

        let mut mesh = Mesh::new(&vertices, &indices, program); // Temporary solution for mesh, because in future we won't draw just squares,
        // TODO: it's need to be rewrite

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

        if let Some(scene) = self.scene_adaptor.get_current_scene_mut() {
            scene.start();
        }

        event_loop.run(move |event, _, control_flow| {
            *control_flow = ControlFlow::Poll;

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
                    if let Some(scene) = self.scene_adaptor.get_current_scene() {
                        renderer.draw(scene, &mut mesh, &textures, &texture_registry);
                    }
                    gl_context.swap_buffers().unwrap();
                }
                _ => (),
            }
        });
    }
}
