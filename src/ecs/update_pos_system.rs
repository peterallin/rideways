use crate::ecs::{Position, Velocity};
use specs::{ReadStorage, System, WriteStorage};

pub struct UpdatePos;

impl<'a> System<'a> for UpdatePos {
    type SystemData = (ReadStorage<'a, Velocity>, WriteStorage<'a, Position>);

    fn run(&mut self, (velocity, mut position): Self::SystemData) {
        use specs::Join;
        for (v, p) in (&velocity, &mut position).join() {
            p.x += v.x;
            p.y += v.y;
        }
    }
}
