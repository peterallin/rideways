use crate::ecs::components::{HarmsAliens, HarmsPlayer, IsAlien, IsPlayer, Position};
use specs::{Entities, ReadStorage, System};

pub struct CollisionChecker;

impl<'a> System<'a> for CollisionChecker {
    type SystemData = (
        Entities<'a>,
        ReadStorage<'a, Position>,
        ReadStorage<'a, IsAlien>,
        ReadStorage<'a, HarmsAliens>,
        ReadStorage<'a, IsPlayer>,
        ReadStorage<'a, HarmsPlayer>,
    );

    fn run(
        &mut self,
        (entities, position, is_alien, harms_aliens, is_player, harms_player): Self::SystemData,
    ) {
        use specs::Join;
        for (harmer_ent, harmer_pos, _) in (&entities, &position, &harms_aliens).join() {
            for (alient_ent, alien_pos, _) in (&entities, &position, &is_alien).join() {
                if alien_pos.rect.overlaps(&harmer_pos.rect) {
                    let _res = entities.delete(alient_ent);
                    let _res = entities.delete(harmer_ent);
                }
            }
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
