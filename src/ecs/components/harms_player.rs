use specs::{Component, NullStorage};
use specs_derive::Component;

#[derive(Debug, Default, Component, Copy, Clone)]
#[storage(NullStorage)] // Might be changed when some entities causes more harm than others
pub struct HarmsPlayer;
