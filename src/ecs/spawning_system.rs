use crate::ecs::{IsAlien, MovementKind, Position, RenderKind, Velocity};
use crate::rect::Rect;
use rand::Rng;
use specs::{Entities, System, WriteStorage};

pub struct Spawning {
    ufo_size: (f32, f32),
}

impl Spawning {
    pub fn new(ufo_size: (f32, f32)) -> Self {
        Spawning { ufo_size }
    }
}

impl<'a> System<'a> for Spawning {
    type SystemData = (
        Entities<'a>,
        WriteStorage<'a, Position>,
        WriteStorage<'a, Velocity>,
        WriteStorage<'a, MovementKind>,
        WriteStorage<'a, RenderKind>,
        WriteStorage<'a, IsAlien>,
    );

    fn run(
        &mut self,
        (entities, mut position, mut velocity, mut movement_kind, mut render_kind, mut is_alien): Self::SystemData,
    ) {
        let mut rng = rand::thread_rng();
        if rng.gen_range(0, 10000) > 9900 {
            // TOOD: Don't hardcode the X and the Y limits, but get arena size from Specs
            let pos = (900.0, rng.gen_range(30.0, 600.0 - self.ufo_size.1));
            entities
                .build_entity()
                .with(
                    Position {
                        rect: Rect::new(pos, self.ufo_size),
                    },
                    &mut position,
                )
                .with(
                    Velocity {
                        x: rng.gen_range(-8.0, -2.0),
                        y: rng.gen_range(-0.5, 0.5),
                    },
                    &mut velocity,
                )
                .with(MovementKind::UFO, &mut movement_kind)
                .with(RenderKind::UFO, &mut render_kind)
                .with(IsAlien, &mut is_alien)
                .build();
        }
    }
}