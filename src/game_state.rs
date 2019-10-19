#[derive(Copy, Clone)]
pub enum GameState {
    Idle { button_pressed: bool },
    Playing { state: PlayingGameState },
    GameOver { seconds_left: f64 },
}

impl GameState {
    pub fn new() -> Self {
        GameState::Idle {
            button_pressed: false,
        }
    }
}

#[derive(Default, Copy, Clone)]
pub struct PlayingGameState {
    pub score: u32,
}

impl PlayingGameState {
    pub fn new() -> PlayingGameState {
        PlayingGameState { score: 0 }
    }
}
