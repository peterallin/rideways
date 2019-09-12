use crate::ecs::components::{
    HarmsAliens, MovementKind, Position, ReapWhenOutside, RenderKind, Velocity,
};
use crate::rect::Rect;
use crate::ControlState;
use specs::{Entities, Read, System, WriteStorage};

pub struct PlayerShooting {
    shot_size: (u32, u32),
    has_shot: bool,
}

impl PlayerShooting {
    pub fn new(shot_size: (u32, u32)) -> Self {
        PlayerShooting {
            shot_size,
            has_shot: false,
        }
    }
}

impl<'a> System<'a> for PlayerShooting {
    type SystemData = (
        Read<'a, ControlState>,
        Entities<'a>,
        WriteStorage<'a, MovementKind>,
        WriteStorage<'a, Position>,
        WriteStorage<'a, Velocity>,
        WriteStorage<'a, RenderKind>,
        WriteStorage<'a, ReapWhenOutside>,
        WriteStorage<'a, HarmsAliens>,
    );

    fn run(
        &mut self,
        (
            control_state,
            entities,
            mut movement_kind,
            mut position,
            mut velocity,
            mut render_kind,
            mut reap_when_outside,
            mut harms_aliens,
        ): Self::SystemData,
    ) {
        use specs::Join;
        let mut fire_positions = vec![];
        for (pos, ()) in (&position, !&movement_kind).join() {
            if !self.has_shot && control_state.fire {
                fire_positions.push(*pos);
                self.has_shot = true;
            }
            if !control_state.fire {
                self.has_shot = false;
            }
        }

        for pos in fire_positions {
            entities
                .build_entity()
                .with(
                    Position {
                        rect: Rect::new(
                            pos.rect.midright(),
                            (self.shot_size.0 as f32, self.shot_size.1 as f32),
                        ),
                    },
                    &mut position,
                )
                .with(Velocity { x: 15.0, y: 0.0 }, &mut velocity)
                .with(RenderKind::BasicShot, &mut render_kind)
                .with(MovementKind::Bullet, &mut movement_kind)
                .with(ReapWhenOutside, &mut reap_when_outside)
                .with(HarmsAliens, &mut harms_aliens)
                .build();
        }
    }
}
