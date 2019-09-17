use crate::ecs::components::{
    HarmsPlayer, IsAlien, IsPlayer, MovementKind, Position, ReapWhenOutside, RenderKind, Velocity,
};
use crate::geometry::{Rect, RectSize};
use rand::Rng;
use specs::{Entities, ReadStorage, System, WriteStorage};

pub struct AlienShooting {
    shot_size: RectSize,
}

impl AlienShooting {
    pub fn new(shot_size: RectSize) -> Self {
        AlienShooting { shot_size }
    }
}

impl<'a> System<'a> for AlienShooting {
    type SystemData = (
        Entities<'a>,
        WriteStorage<'a, MovementKind>,
        WriteStorage<'a, Position>,
        WriteStorage<'a, Velocity>,
        WriteStorage<'a, RenderKind>,
        WriteStorage<'a, ReapWhenOutside>,
        WriteStorage<'a, HarmsPlayer>,
        ReadStorage<'a, IsAlien>,
        ReadStorage<'a, IsPlayer>,
    );

    fn run(
        &mut self,
        (
            entities,
            mut movement_kind,
            mut position,
            mut velocity,
            mut render_kind,
            mut reap_when_outside,
            mut harms_player,
            is_alien,
            is_player,
        ): Self::SystemData,
    ) {
        let mut rng = rand::thread_rng();

        use specs::Join;
        let mut fire_positions = vec![];
        for (pos, _) in (&position, &is_alien).join() {
            if rng.gen_range(0, 10000) > 9900 {
                fire_positions.push(*pos);
            }
        }

        let player_pos = (&mut position, &is_player).join().nth(0).map(|x| *x.0);

        if let Some(player_pos) = player_pos {
            for pos in fire_positions {
                let diff = (
                    player_pos.rect.center().0 - pos.rect.center().0,
                    player_pos.rect.center().1 - pos.rect.center().1,
                );
                let diff_size = (diff.0.powf(2.0) + diff.1.powf(2.0)).sqrt();
                let speed = 500.0;
                let vel = (speed * diff.0 / diff_size, speed * diff.1 / diff_size);
                let shot_pos = pos.rect.center();
                entities
                    .build_entity()
                    .with(
                        Position {
                            rect: Rect::new(shot_pos, self.shot_size),
                        },
                        &mut position,
                    )
                    .with(Velocity { x: vel.0, y: vel.1 }, &mut velocity)
                    .with(RenderKind::UFOShot, &mut render_kind)
                    .with(MovementKind::Bullet, &mut movement_kind)
                    .with(ReapWhenOutside, &mut reap_when_outside)
                    .with(HarmsPlayer, &mut harms_player)
                    .build();
            }
        }
    }
}
