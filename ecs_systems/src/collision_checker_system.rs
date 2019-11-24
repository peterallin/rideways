use ecs_components::{
    HarmsAliens, HarmsPlayer, Invincibility, IsAlien, IsExplosion, IsPlayer, Lifetime, Position,
    SpawnerKind,
};
use geometry::Rect;
use shared_types::PlayingGameState;

use specs::{Entities, ReadStorage, System, Write, WriteStorage};

pub struct CollisionChecker;

impl<'a> System<'a> for CollisionChecker {
    type SystemData = (
        Entities<'a>,
        Write<'a, PlayingGameState>,
        WriteStorage<'a, Position>,
        WriteStorage<'a, SpawnerKind>,
        WriteStorage<'a, Lifetime>,
        WriteStorage<'a, IsExplosion>,
        ReadStorage<'a, IsAlien>,
        ReadStorage<'a, HarmsAliens>,
        ReadStorage<'a, IsPlayer>,
        ReadStorage<'a, HarmsPlayer>,
        ReadStorage<'a, Invincibility>,
    );

    fn run(
        &mut self,
        (
            entities,
            mut active_game_state,
            mut position,
            mut spawner_kind,
            mut lifetime,
            mut is_explosion,
            is_alien,
            harms_aliens,
            is_player,
            harms_player,
            invincibility,
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
                .with(SpawnerKind::Fire(0.2), &mut spawner_kind)
                .with(Lifetime { seconds: 0.1 }, &mut lifetime)
                .with(IsExplosion, &mut is_explosion)
                .build();
        }

        let mut dead_player_position = None;
        for (harmer_ent, harmer_pos, _) in (&entities, &position, &harms_player).join() {
            for (player_ent, player_pos, _, _) in
                (&entities, &position, &is_player, !&invincibility).join()
            {
                if player_pos.rect.overlaps(&harmer_pos.rect) {
                    let _res = entities.delete(harmer_ent);
                    let _res = entities.delete(player_ent);
                    dead_player_position = Some((player_pos.rect.left(), player_pos.rect.top()));
                }
            }
        }
        if let Some(dead_player_position) = dead_player_position {
            entities
                .build_entity()
                .with(
                    Position {
                        rect: Rect::new(dead_player_position.into(), (0, 0).into()),
                    },
                    &mut position,
                )
                .with(SpawnerKind::Fire(0.5), &mut spawner_kind)
                .with(Lifetime { seconds: 0.5 }, &mut lifetime)
                .with(IsExplosion, &mut is_explosion)
                .build();
        }
    }
}
