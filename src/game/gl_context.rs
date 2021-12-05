use glium::glutin;

pub struct GLContext<Msg>
where
    Msg: 'static,
{
    pub event_loop: glutin::event_loop::EventLoop<Msg>,
    pub display: glium::Display,
    pub starting_scale_factor: f64,
}

impl<G> Default for GLContext<G> {
    /// Set up OpenGL in a sane testing configuration
    fn default() -> Self {
        let event_loop = glutin::event_loop::EventLoop::with_user_event();
        let wb = glutin::window::WindowBuilder::new()
            .with_inner_size(glutin::dpi::LogicalSize::new(500, 500))
            .with_title("Hello world!");
        let wc = glutin::ContextBuilder::new()
            .build_windowed(wb, &event_loop)
            .expect("should be able to build windowed context");
        let starting_scale_factor = wc.window().scale_factor();
        // Used for building buffers, textures, and drawing
        let display = glium::Display::from_gl_window(wc).expect("Could not build display");
        GLContext {
            event_loop,
            display,
            starting_scale_factor,
        }
    }
}
