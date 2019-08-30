use crate::ecs::render_kind::RenderKind;
use crate::ecs::{MovementKind, Position, Velocity};
use crate::rect::Rect;
use crate::ControlState;
use specs::{Entities, Read, System, WriteStorage};

pub struct PlayerControl;

impl<'a> System<'a> for PlayerControl {
    type SystemData = (
        Read<'a, ControlState>,
        Entities<'a>,
        WriteStorage<'a, MovementKind>,
        WriteStorage<'a, Position>,
        WriteStorage<'a, Velocity>,
        WriteStorage<'a, RenderKind>,
    );

    fn run(
        &mut self,
        (control_state, entities, mut movement_kind, mut position, mut velocity, mut render_kind): Self::SystemData,
    ) {
        use specs::Join;
        let mut fire_positions = vec![];
        for (vel, pos, ()) in (&mut velocity, &position, !&movement_kind).join() {
            if control_state.left {
                vel.x = -4.0;
            }
            if control_state.right {
                vel.x = 4.0;
            }
            if !control_state.left && !control_state.right {
                vel.x = 0.0;
            }
            if control_state.up {
                vel.y = -4.0;
            }
            if control_state.down {
                vel.y = 4.0;
            }
            if !control_state.up && !control_state.down {
                vel.y = 0.0;
            }
            if control_state.fire {
                fire_positions.push(*pos);
            }
        }
        for pos in fire_positions {
            entities
                .build_entity()
                .with(
                    Position {
                        rect: Rect::new((pos.rect.left(), pos.rect.top()), (32.0, 32.0)), // TODO: Not left, top but mid-right, do not hardcode 32, 32
                    },
                    &mut position,
                )
                .with(Velocity { x: 20.0, y: 0.0 }, &mut velocity)
                .with(RenderKind::BasicShot, &mut render_kind)
                .with(MovementKind::Bullet, &mut movement_kind)
                .build();
        }
    }
}
