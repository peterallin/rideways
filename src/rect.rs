#![allow(dead_code)] // TODO: Remove

#[derive(Debug, Copy, Clone, Default)]
pub struct RectSize(pub f32, pub f32);

impl From<(u32, u32)> for RectSize {
    fn from((w, h): (u32, u32)) -> Self {
        RectSize(w as f32, h as f32)
    }
}

#[derive(Debug, Copy, Clone, Default)]
pub struct Rect {
    left: f32,
    top: f32,
    size: RectSize,
}

impl Rect {
    pub fn new((left, top): (f32, f32), size: RectSize) -> Self {
        Rect { left, top, size }
    }

    pub fn set_left(&mut self, new_left: f32) {
        self.left = new_left;
    }

    pub fn set_top(&mut self, new_top: f32) {
        self.top = new_top;
    }

    pub fn set_right(&mut self, new_right: f32) {
        self.left = new_right - self.width();
    }

    pub fn set_bottom(&mut self, new_bottom: f32) {
        self.top = new_bottom - self.height();
    }

    pub fn left(&self) -> f32 {
        self.left
    }

    pub fn top(&self) -> f32 {
        self.top
    }

    pub fn width(&self) -> f32 {
        self.size.0
    }

    pub fn height(&self) -> f32 {
        self.size.1
    }

    pub fn bottom(&self) -> f32 {
        self.top + self.size.1
    }

    pub fn right(&self) -> f32 {
        self.left + self.size.0
    }

    pub fn midtop(&self) -> (f32, f32) {
        (self.left + self.size.0 / 2.0, self.top)
    }

    pub fn midbottom(&self) -> (f32, f32) {
        (self.left + self.size.0 / 2.0, self.top + self.size.1)
    }

    pub fn midright(&self) -> (f32, f32) {
        (self.left + self.size.0, self.top + self.size.1 / 2.0)
    }

    pub fn midleft(&self) -> (f32, f32) {
        (self.left, self.top + self.size.1 / 2.0)
    }

    pub fn center(&self) -> (f32, f32) {
        (
            self.left + self.width() / 2.0,
            self.top + self.height() / 2.0,
        )
    }

    pub fn r#move(&mut self, dx: f32, dy: f32) {
        self.left += dx;
        self.top += dy;
    }

    pub fn overlaps(&self, other: &Rect) -> bool {
        !(self.left() > other.right()
            || other.left() > self.right()
            || self.top() > other.bottom()
            || other.top() > self.bottom())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_inputs_are_stored() {
        let rect = Rect::new((100.0, 200.0), (20, 50).into());
        assert_eq!(rect.left(), 100.0);
        assert_eq!(rect.top(), 200.0);
        assert_eq!(rect.width(), 20.0);
        assert_eq!(rect.height(), 50.0);
    }

    #[test]
    fn test_calculated_values() {
        let rect = Rect::new((100.0, 200.0), (20, 50).into());
        assert_eq!(rect.right(), 120.0);
        assert_eq!(rect.bottom(), 250.0);
    }

    #[test]
    fn test_middle_functions() {
        let rect = Rect::new((100.0, 200.0), (20, 50).into());
        assert_eq!(rect.midleft(), (100.0, 225.0));
        assert_eq!(rect.midright(), (120.0, 225.0));
        assert_eq!(rect.midtop(), (110.0, 200.0));
        assert_eq!(rect.midbottom(), (110.0, 250.0));
    }
}
