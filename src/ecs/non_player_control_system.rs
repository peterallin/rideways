use crate::ecs::{MovementKind, Position, Velocity};
use specs::{ReadStorage, System, WriteStorage};

pub struct NonPlayerControl;

fn control_ufo(position: &Position, velocity: &mut Velocity) {
    // TODO: Get the bounding rect from Specs, and have a rect for the position
    if position.rect.left() <= 0.0 {
        velocity.x = velocity.x.abs();
    } else if position.rect.right() > 800.0 {
        velocity.x = -velocity.x.abs();
    }

    if position.rect.top() <= 0.0 {
        velocity.y = velocity.y.abs();
    } else if position.rect.bottom() >= 600.0 {
        velocity.y = -velocity.y.abs();
    }
}

impl<'a> System<'a> for NonPlayerControl {
    type SystemData = (
        ReadStorage<'a, MovementKind>,
        ReadStorage<'a, Position>,
        WriteStorage<'a, Velocity>,
    );

    fn run(&mut self, (kind, pos, mut vel): Self::SystemData) {
        use specs::Join;
        for (kind, pos, vel) in (&kind, &pos, &mut vel).join() {
            match kind {
                MovementKind::UFO => control_ufo(pos, vel),
                MovementKind::Bullet => {}
            }
        }
    }
}
