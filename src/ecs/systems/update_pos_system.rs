use crate::ecs::components::{Position, Velocity};
use specs::{ReadStorage, System, WriteStorage};

pub struct UpdatePos;

impl<'a> System<'a> for UpdatePos {
    type SystemData = (ReadStorage<'a, Velocity>, WriteStorage<'a, Position>);

    fn run(&mut self, (velocity, mut position): Self::SystemData) {
        use specs::Join;
        for (v, p) in (&velocity, &mut position).join() {
            p.rect.r#move(v.x, v.y);
        }
    }
}
