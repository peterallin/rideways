use crate::ecs::components::{IsAlien, MovementKind, Position, RenderKind, Velocity};
use crate::rect::{Rect, RectSize};
use crate::Arena;
use rand::Rng;
use specs::{Entities, Read, System, WriteStorage};

pub struct Spawning {
    ufo_size: RectSize,
}

impl Spawning {
    pub fn new(ufo_size: RectSize) -> Self {
        Spawning { ufo_size }
    }
}

impl<'a> System<'a> for Spawning {
    type SystemData = (
        Entities<'a>,
        Read<'a, Arena>,
        WriteStorage<'a, Position>,
        WriteStorage<'a, Velocity>,
        WriteStorage<'a, MovementKind>,
        WriteStorage<'a, RenderKind>,
        WriteStorage<'a, IsAlien>,
    );

    fn run(
        &mut self,
        (
            entities,
            arena,
            mut position,
            mut velocity,
            mut movement_kind,
            mut render_kind,
            mut is_alien,
        ): Self::SystemData,
    ) {
        let arena_rect = arena.0;
        let mut rng = rand::thread_rng();
        if rng.gen_range(0, 10000) > 9900 {
            let pos = (
                arena_rect.right() + 100.0,
                rng.gen_range(arena_rect.top(), arena_rect.bottom() - self.ufo_size.1),
            );
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
                        x: rng.gen_range(-600.0, -200.0),
                        y: rng.gen_range(-35.0, 35.0),
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
