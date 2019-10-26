use ecs_components::{Draw, Position, ReapWhenOutside, Velocity};
use geometry::{Rect, RectSize};
use rand::Rng;
use shared_types::Arena;
use specs::{Entities, Read, System, WriteStorage};

pub struct StarSpawner;

impl<'a> System<'a> for StarSpawner {
    type SystemData = (
        Entities<'a>,
        Read<'a, Arena>,
        WriteStorage<'a, Position>,
        WriteStorage<'a, Velocity>,
        WriteStorage<'a, Draw>,
        WriteStorage<'a, ReapWhenOutside>,
    );

    fn run(
        &mut self,
        (
            entities,
            arena,
            mut position,
            mut velocity,
            mut draw,
            mut reap_when_outside,
        ): Self::SystemData,
    ) {
        let arena_rect = arena.0;
        let mut rng = rand::thread_rng();
        if rng.gen_range(0, 10000) > 9500 {
            let x = arena_rect.right() - 1.0;
            let y = rng.gen_range(arena_rect.top(), arena_rect.bottom());
            let pos = (x, y).into();
            let radius = rng.gen_range(1, 5);
            let speed = rng.gen_range(15, 50);
            entities
                .build_entity()
                .with(
                    Position {
                        rect: Rect::new(pos, RectSize(0.0, 0.0)),
                    },
                    &mut position,
                )
                .with(
                    Velocity {
                        x: -speed as f32,
                        y: 0.0,
                    },
                    &mut velocity,
                )
                .with(Draw::Star(radius), &mut draw)
                .with(ReapWhenOutside, &mut reap_when_outside)
                .build();
        }
    }
}
