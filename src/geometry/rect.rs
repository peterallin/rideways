#![allow(dead_code)] // TODO: Remove

use crate::geometry::{Position, RectSize};

#[derive(Debug, Copy, Clone, Default)]
pub struct Rect {
    topleft: Position,
    size: RectSize,
}

impl Rect {
    pub fn new(topleft: Position, size: RectSize) -> Self {
        Rect { topleft, size }
    }

    pub fn set_left(&mut self, new_left: f32) {
        self.topleft.0 = new_left;
    }

    pub fn set_top(&mut self, new_top: f32) {
        self.topleft.1 = new_top;
    }

    pub fn set_right(&mut self, new_right: f32) {
        self.topleft.0 = new_right - self.width();
    }

    pub fn set_bottom(&mut self, new_bottom: f32) {
        self.topleft.1 = new_bottom - self.height();
    }

    pub fn left(&self) -> f32 {
        self.topleft.0
    }

    pub fn top(&self) -> f32 {
        self.topleft.1
    }

    pub fn width(&self) -> f32 {
        self.size.0
    }

    pub fn height(&self) -> f32 {
        self.size.1
    }

    pub fn bottom(&self) -> f32 {
        self.topleft.1 + self.size.1
    }

    pub fn right(&self) -> f32 {
        self.topleft.0 + self.size.0
    }

    pub fn midtop(&self) -> Position {
        (self.topleft.0 + self.size.0 / 2.0, self.topleft.1).into()
    }

    pub fn midbottom(&self) -> Position {
        (
            self.topleft.0 + self.size.0 / 2.0,
            self.topleft.1 + self.size.1,
        )
            .into()
    }

    pub fn midright(&self) -> Position {
        (
            self.topleft.0 + self.size.0,
            self.topleft.1 + self.size.1 / 2.0,
        )
            .into()
    }

    pub fn midleft(&self) -> Position {
        (self.topleft.0, self.topleft.1 + self.size.1 / 2.0).into()
    }

    pub fn center(&self) -> Position {
        (
            self.topleft.0 + self.width() / 2.0,
            self.topleft.1 + self.height() / 2.0,
        )
            .into()
    }

    pub fn r#move(&mut self, dx: f32, dy: f32) {
        self.topleft.0 += dx;
        self.topleft.1 += dy;
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
        let rect = Rect::new((100.0, 200.0).into(), (20, 50).into());
        assert_eq!(rect.left(), 100.0);
        assert_eq!(rect.top(), 200.0);
        assert_eq!(rect.width(), 20.0);
        assert_eq!(rect.height(), 50.0);
    }

    #[test]
    fn test_calculated_values() {
        let rect = Rect::new((100.0, 200.0).into(), (20, 50).into());
        assert_eq!(rect.right(), 120.0);
        assert_eq!(rect.bottom(), 250.0);
    }

    #[test]
    fn test_middle_functions() {
        let rect = Rect::new((100.0, 200.0).into(), (20, 50).into());
        assert_eq!(rect.midleft(), (100.0, 225.0).into());
        assert_eq!(rect.midright(), (120.0, 225.0).into());
        assert_eq!(rect.midtop(), (110.0, 200.0).into());
        assert_eq!(rect.midbottom(), (110.0, 250.0).into());
    }
}
