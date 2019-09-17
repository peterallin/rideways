use crate::ecs::components::{Position, Velocity};
use crate::ElapsedSeconds;
use specs::{Read, ReadStorage, System, WriteStorage};

pub struct UpdatePos;

impl<'a> System<'a> for UpdatePos {
    type SystemData = (
        Read<'a, ElapsedSeconds>,
        ReadStorage<'a, Velocity>,
        WriteStorage<'a, Position>,
    );

    fn run(&mut self, (delta_time, velocity, mut position): Self::SystemData) {
        use specs::Join;
        for (v, p) in (&velocity, &mut position).join() {
            p.rect.r#move(v.x * delta_time.0, v.y * delta_time.0);
        }
    }
}
