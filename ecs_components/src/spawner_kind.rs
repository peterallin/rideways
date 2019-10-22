use specs::{Component, VecStorage};
use specs_derive::Component;

#[derive(Debug, Component, Copy, Clone)]
#[storage(VecStorage)]
pub enum SpawnerKind {
    Fire,
}
