use crate::ecs::components::{MovementKind, Position, Velocity};
use crate::Arena;
use specs::{Read, ReadStorage, System, WriteStorage};

pub struct NonPlayerControl;

fn control_ufo(position: &Position, velocity: &mut Velocity, arena: &Arena) {
    let arena_rect = arena.0;
    if position.rect.left() <= arena_rect.left() {
        velocity.x = velocity.x.abs();
    } else if position.rect.right() > arena_rect.right() {
        velocity.x = -velocity.x.abs();
    }

    if position.rect.top() <= arena_rect.top() {
        velocity.y = velocity.y.abs();
    } else if position.rect.bottom() >= arena_rect.bottom() {
        velocity.y = -velocity.y.abs();
    }
}

impl<'a> System<'a> for NonPlayerControl {
    type SystemData = (
        Read<'a, Arena>,
        ReadStorage<'a, MovementKind>,
        ReadStorage<'a, Position>,
        WriteStorage<'a, Velocity>,
    );

    fn run(&mut self, (arena, kind, pos, mut vel): Self::SystemData) {
        use specs::Join;
        for (kind, pos, vel) in (&kind, &pos, &mut vel).join() {
            match kind {
                MovementKind::SideToSide => control_ufo(pos, vel, &*arena),
            }
        }
    }
}
