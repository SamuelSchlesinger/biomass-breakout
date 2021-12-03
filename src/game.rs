pub struct GameContext {}

impl GameContext {
    pub fn new() -> Self {
        GameContext {}
    }

    pub fn render(&self, display: &mut glium::Display) {
        use glium::Surface;
        let mut target = display.draw();
        target.clear_color(0.0, 0.0, 0.0, 0.0);
        target.finish().expect("finish no work?");
    }

    pub fn press(&mut self, virtual_key: glium::glutin::event::VirtualKeyCode) {}
}
