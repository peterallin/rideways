use crate::ecs::components::{
    HarmsAliens, HarmsPlayer, IsAlien, IsPlayer, Lifetime, Position, SpawnerKind,
};
use crate::geometry::Rect;
use crate::PlayingGameState;

use specs::{Entities, ReadStorage, System, Write, WriteStorage};

pub struct CollisionChecker;

impl<'a> System<'a> for CollisionChecker {
    type SystemData = (
        Entities<'a>,
        Write<'a, PlayingGameState>,
        WriteStorage<'a, Position>,
        WriteStorage<'a, SpawnerKind>,
        WriteStorage<'a, Lifetime>,
        ReadStorage<'a, IsAlien>,
        ReadStorage<'a, HarmsAliens>,
        ReadStorage<'a, IsPlayer>,
        ReadStorage<'a, HarmsPlayer>,
    );

    fn run(
        &mut self,
        (
            entities,
            mut active_game_state,
            mut position,
            mut spawner_kind,
            mut lifetime,
            is_alien,
            harms_aliens,
            is_player,
            harms_player,
        ): Self::SystemData,
    ) {
        use specs::Join;
        let mut explosion_positions = vec![];
        for (harmer_ent, harmer_pos, _) in (&entities, &position, &harms_aliens).join() {
            for (alient_ent, alien_pos, _) in (&entities, &position, &is_alien).join() {
                if alien_pos.rect.overlaps(&harmer_pos.rect) {
                    let _res = entities.delete(alient_ent);
                    let _res = entities.delete(harmer_ent);
                    explosion_positions.push((alien_pos.rect.left(), alien_pos.rect.top()));
                    active_game_state.score += 1;
                }
            }
        }

        for explosion_position in explosion_positions {
            entities
                .build_entity()
                .with(
                    Position {
                        rect: Rect::new(explosion_position.into(), (0, 0).into()),
                    },
                    &mut position,
                )
                .with(SpawnerKind::Fire, &mut spawner_kind)
                .with(Lifetime { seconds: 0.1 }, &mut lifetime)
                .build();
        }

        for (harmer_ent, harmer_pos, _) in (&entities, &position, &harms_player).join() {
            for (player_ent, player_pos, _) in (&entities, &position, &is_player).join() {
                if player_pos.rect.overlaps(&harmer_pos.rect) {
                    let _res = entities.delete(harmer_ent);
                    let _res = entities.delete(player_ent);
                }
            }
        }
    }
}
