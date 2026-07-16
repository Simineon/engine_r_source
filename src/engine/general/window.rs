use glutin::ContextWrapper;
use glutin::PossiblyCurrent;
use glutin::event_loop::EventLoop;
use glutin::window::WindowBuilder;
use glutin::{Api, ContextBuilder, GlRequest};

pub struct Window {
    pub gl_context: ContextWrapper<PossiblyCurrent, glutin::window::Window>,
}

impl Window {
    pub fn new(title: &str, event_loop: &EventLoop<()>) -> Self {
        let window_builder = WindowBuilder::new().with_title(title);

        let gl_context = ContextBuilder::new()
            .with_gl(GlRequest::Specific(Api::OpenGl, (3, 3)))
            .build_windowed(window_builder, event_loop)
            .expect("Cannot create windowed context");

        let gl_context = unsafe {
            gl_context
                .make_current()
                .expect("Failed to make context current")
        };

        gl::load_with(|ptr| gl_context.get_proc_address(ptr) as *const _);

        Window { gl_context }
    }
}
