use specs::{Component, VecStorage};
use specs_derive::Component;

#[derive(Debug, Component)]
#[storage(VecStorage)]
pub struct Lifetime {
    pub seconds: f32,
}
