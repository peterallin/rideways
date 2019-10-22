#[derive(Debug, Copy, Clone, Default)]
pub struct RectSize(pub f32, pub f32);

impl From<(u32, u32)> for RectSize {
    fn from((w, h): (u32, u32)) -> Self {
        RectSize(w as f32, h as f32)
    }
}
