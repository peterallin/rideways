#[derive(Debug, Copy, Clone, Default, PartialEq)]
pub struct Position(pub f32, pub f32);

impl From<(u32, u32)> for Position {
    fn from((x, y): (u32, u32)) -> Self {
        Position(x as f32, y as f32)
    }
}

impl From<(f32, f32)> for Position {
    fn from((x, y): (f32, f32)) -> Self {
        Position(x, y)
    }
}
