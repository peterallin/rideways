use specs::{Component, VecStorage};
use specs_derive::Component;

#[derive(Debug, Component)]
#[storage(VecStorage)]
pub struct Position {
    pub x: f32,
    pub y: f32,
}
