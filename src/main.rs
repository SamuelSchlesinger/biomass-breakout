fn main() {
    let mut events_loop = glium::glutin::event_loop::EventLoop::new();
    let wb = glium::glutin::window::WindowBuilder::new()
        .with_inner_size(glium::glutin::dpi::LogicalSize::new(500, 500))
        .with_title("Hello world!");
    let cb = glium::glutin::ContextBuilder::new();
    // Used for building buffers, textures, and drawing
    let display = glium::Display::new(wb, cb, &events_loop).expect("Could not build display");

}
