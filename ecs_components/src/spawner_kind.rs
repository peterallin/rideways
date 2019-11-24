use specs::{Component, VecStorage};
use specs_derive::Component;

type SpawnerLifetime = f32;

#[derive(Debug, Component, Copy, Clone)]
#[storage(VecStorage)]
pub enum SpawnerKind {
    Fire(SpawnerLifetime),
}
