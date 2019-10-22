use ecs_components::{Position, ReapWhenOutside};
use shared_types::Arena;
use specs::{Entities, Read, ReadStorage, System};

pub struct ReapOutsiders;

impl<'a> System<'a> for ReapOutsiders {
    type SystemData = (
        Entities<'a>,
        Read<'a, Arena>,
        ReadStorage<'a, Position>,
        ReadStorage<'a, ReapWhenOutside>,
    );

    fn run(&mut self, (entities, arena, position, reap_when_outside): Self::SystemData) {
        use specs::Join;
        for (ent, pos, _) in (&entities, &position, &reap_when_outside).join() {
            if !pos.rect.overlaps(&arena.0) {
                let _res = entities.delete(ent);
            }
        }
    }
}
