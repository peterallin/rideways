use ecs_components::{HarmsAliens, IsPlayer, Position, ReapWhenOutside, RenderKind, Velocity};
use geometry::Rect;
use sdl_input::ControlState;
use specs::{Entities, Read, ReadStorage, System, WriteStorage};

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
        WriteStorage<'a, Position>,
        WriteStorage<'a, Velocity>,
        WriteStorage<'a, RenderKind>,
        WriteStorage<'a, ReapWhenOutside>,
        WriteStorage<'a, HarmsAliens>,
        ReadStorage<'a, IsPlayer>,
    );

    fn run(
        &mut self,
        (
            control_state,
            entities,
            mut position,
            mut velocity,
            mut render_kind,
            mut reap_when_outside,
            mut harms_aliens,
            is_player,
        ): Self::SystemData,
    ) {
        use specs::Join;
        let mut fire_positions = vec![];
        for (pos, _) in (&position, &is_player).join() {
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
                        rect: Rect::new(pos.rect.midright(), self.shot_size.into()),
                    },
                    &mut position,
                )
                .with(Velocity { x: 1500.0, y: 0.0 }, &mut velocity)
                .with(RenderKind::BasicShot, &mut render_kind)
                .with(ReapWhenOutside, &mut reap_when_outside)
                .with(HarmsAliens, &mut harms_aliens)
                .build();
        }
    }
}
