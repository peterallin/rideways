use specs::{Component, VecStorage};
use specs_derive::Component;

#[derive(Debug, Component)]
#[storage(VecStorage)]
pub enum Draw {
    Star(i16),
}
