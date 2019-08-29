#![allow(dead_code)] // TODO: Remove

#[derive(Debug)]
pub struct Rect {
    left: f32,
    top: f32,
    width: f32,
    height: f32,
}

impl Rect {
    pub fn new((left, top): (f32, f32), (width, height): (f32, f32)) -> Self {
        Rect {
            left,
            top,
            width: width,
            height: height,
        }
    }

    pub fn left(&self) -> f32 {
        self.left
    }

    pub fn top(&self) -> f32 {
        self.top
    }

    pub fn width(&self) -> f32 {
        self.width
    }

    pub fn height(&self) -> f32 {
        self.height
    }

    pub fn bottom(&self) -> f32 {
        self.top + self.height
    }

    pub fn right(&self) -> f32 {
        self.left + self.width
    }

    pub fn r#move(&mut self, dx: f32, dy: f32) {
        self.left += dx;
        self.top += dy;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_inputs_are_stored() {
        let rect = Rect::new((100.0, 200.0), (20.0, 50.0));
        assert_eq!(rect.left(), 100.0);
        assert_eq!(rect.top(), 200.0);
        assert_eq!(rect.width(), 20.0);
        assert_eq!(rect.height(), 50.0);
    }

    #[test]
    fn test_calculated_values() {
        let rect = Rect::new((100.0, 200.0), (20.0, 50.0));
        assert_eq!(rect.right(), 120.0);
        assert_eq!(rect.bottom(), 250.0);
    }
}
