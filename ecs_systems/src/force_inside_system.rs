use ecs_components::{KeepInside, Position};
use shared_types::Arena;
use specs::{Read, ReadStorage, System, WriteStorage};

pub struct ForceInside;

impl<'a> System<'a> for ForceInside {
    type SystemData = (
        Read<'a, Arena>,
        WriteStorage<'a, Position>,
        ReadStorage<'a, KeepInside>,
    );

    fn run(&mut self, (arena, mut position, keep_inside): Self::SystemData) {
        use specs::Join;
        for (pos, _) in (&mut position, &keep_inside).join() {
            if pos.rect.left() < arena.0.left() {
                pos.rect.set_left(arena.0.left());
            }

            if pos.rect.top() < arena.0.top() {
                pos.rect.set_top(arena.0.top());
            }

            if pos.rect.right() > arena.0.right() {
                pos.rect.set_right(arena.0.right());
            }

            if pos.rect.bottom() > arena.0.bottom() {
                pos.rect.set_bottom(arena.0.bottom())
            }
        }
    }
}
