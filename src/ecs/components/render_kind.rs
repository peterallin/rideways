use specs::{Component, VecStorage};
use specs_derive::Component;

#[derive(Debug, Component, PartialEq, PartialOrd, Eq, Ord)]
#[storage(VecStorage)]
pub enum RenderKind {
    Player,
    BasicShot,
    UFO,
    UFOShot,
    Glow,
}
