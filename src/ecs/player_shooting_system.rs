use crate::ecs::render_kind::RenderKind;
use crate::ecs::{MovementKind, Position, Velocity};
use crate::rect::Rect;
use crate::ControlState;
use specs::{Entities, Read, System, WriteStorage};

pub struct PlayerShooting {
    pub shot_size: (u32, u32),
}

impl<'a> System<'a> for PlayerShooting {
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
        for (pos, ()) in (&position, !&movement_kind).join() {
            if control_state.fire {
                fire_positions.push(*pos);
            }
        }

        for pos in fire_positions {
            entities
                .build_entity()
                .with(
                    Position {
                        rect: Rect::new(
                            (pos.rect.left(), pos.rect.top()),
                            (self.shot_size.0 as f32, self.shot_size.1 as f32),
                        ), // TODO: Not left, top but mid-right
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
