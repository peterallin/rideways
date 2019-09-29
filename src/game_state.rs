#[derive(Copy, Clone)]
pub enum GameState {
    Idle { button_pressed: bool },
    Playing,
    GameOver,
}
