mod entity_sizes;
use geometry::Rect;

pub use entity_sizes::EntitySizes;

#[derive(Default, Debug)]
pub struct ElapsedSeconds(pub f32);

#[derive(Default)]
pub struct Arena(pub Rect);

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
    pub lives_left: i32,
}

impl PlayingGameState {
    pub fn new() -> PlayingGameState {
        PlayingGameState {
            score: 0,
            lives_left: 3,
        }
    }

    pub fn any_lives_left(&self) -> bool {
        self.lives_left > 0
    }

    pub fn one_dead(&mut self) {
        self.lives_left -= 1;
    }
}
