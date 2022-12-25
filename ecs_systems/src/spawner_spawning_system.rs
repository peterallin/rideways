use ecs_components::{IsExplosion, Lifetime, Position, SpawnerKind, Sprite, Velocity};
use geometry::Rect;
use rand::Rng;
use specs::{Entities, ReadStorage, System, WriteStorage};

pub struct SpawnerSpawning;

impl<'a> System<'a> for SpawnerSpawning {
    type SystemData = (
        Entities<'a>,
        ReadStorage<'a, SpawnerKind>,
        WriteStorage<'a, Position>,
        WriteStorage<'a, Velocity>,
        WriteStorage<'a, Sprite>,
        WriteStorage<'a, Lifetime>,
        WriteStorage<'a, IsExplosion>,
    );

    fn run(
        &mut self,
        (
            entities,
            spawner_kind,
            mut position,
            mut velocity,
            mut sprite,
            mut lifetime,
            mut is_explosion,
        ): Self::SystemData,
    ) {
        use specs::Join;
        let mut rng = rand::thread_rng();
        let mut spawns = vec![];
        for (kind, pos) in (&spawner_kind, &position).join() {
            for _ in 0..10 {
                spawns.push((kind, pos.rect.center()))
            }
        }

        for (kind, pos) in spawns {
            match kind {
                SpawnerKind::Fire(glow_lifetime) => {
                    let speed = rng.gen_range(100..400) as f32;
                    let direction = rng.gen_range(0..360) as f32 / 180.0 * std::f32::consts::PI;
                    let velocity_x = speed * direction.cos();
                    let velocity_y = -speed * direction.sin();
                    entities
                        .build_entity()
                        .with(Sprite::Glow, &mut sprite)
                        .with(
                            Position {
                                rect: Rect::new(pos, (1, 1).into()),
                            },
                            &mut position,
                        )
                        .with(
                            Velocity {
                                x: velocity_x,
                                y: velocity_y,
                            },
                            &mut velocity,
                        )
                        .with(
                            Lifetime {
                                seconds: *glow_lifetime,
                            },
                            &mut lifetime,
                        )
                        .with(IsExplosion, &mut is_explosion)
                        .build();
                    {}
                }
            };
        }
    }
}
