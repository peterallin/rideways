#[derive(Debug, Copy, Clone, Default)]
pub struct ControlState {
    pub left: bool,
    pub right: bool,
    pub up: bool,
    pub down: bool,
    pub fire: bool,
}

impl ControlState {
    pub fn new() -> Self {
        ControlState {
            left: false,
            right: false,
            up: false,
            down: false,
            fire: false,
        }
    }

    pub fn update(&mut self, event: &sdl2::event::Event) {
        match event {
            sdl2::event::Event::KeyUp {
                keycode: Some(key), ..
            } => match key {
                sdl2::keyboard::Keycode::W => self.up = false,
                sdl2::keyboard::Keycode::A => self.left = false,
                sdl2::keyboard::Keycode::S => self.down = false,
                sdl2::keyboard::Keycode::D => self.right = false,
                sdl2::keyboard::Keycode::Return => self.fire = false,
                _ => {}
            },
            sdl2::event::Event::KeyDown {
                keycode: Some(key), ..
            } => match key {
                sdl2::keyboard::Keycode::W => self.up = true,
                sdl2::keyboard::Keycode::A => self.left = true,
                sdl2::keyboard::Keycode::S => self.down = true,
                sdl2::keyboard::Keycode::D => self.right = true,
                sdl2::keyboard::Keycode::Return => self.fire = true,
                _ => {}
            },
            _ => {}
        }
    }
}
