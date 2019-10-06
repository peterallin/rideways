#[derive(Copy, Clone)]
pub enum GameState {
    Idle { button_pressed: bool },
    Playing,
    GameOver { seconds_left: f64 },
}

impl GameState {
    pub fn new() -> Self {
        GameState::Idle {
            button_pressed: false,
        }
    }
}
