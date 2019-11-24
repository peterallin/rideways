use specs::{Component, NullStorage};
use specs_derive::Component;

#[derive(Debug, Component, Default)]
#[storage(NullStorage)]
pub struct IsExplosion;
