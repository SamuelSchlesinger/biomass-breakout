pub struct GameContext {}

impl GameContext {
    pub fn new() -> Self {
        GameContext {}
    }

    pub fn render(&self, target: &mut glium::Frame) {
        use glium::Surface;
        target.clear_color(1.0, 0.5, 0.5, 0.5);
    }

    pub fn press(&mut self, virtual_key: glium::glutin::event::VirtualKeyCode) {}
}
