use ecs_components::{Invincibility, RenderKind};
use shared_types::ElapsedSeconds;
use specs::{Entities, LazyUpdate, Read, System, WriteStorage};

pub struct InvincibilityWatching;

impl<'a> System<'a> for InvincibilityWatching {
    type SystemData = (
        Entities<'a>,
        Read<'a, LazyUpdate>,
        Read<'a, ElapsedSeconds>,
        WriteStorage<'a, Invincibility>,
    );

    fn run(&mut self, (entitites, updater, delta_time, mut invincibility): Self::SystemData) {
        use specs::Join;
        for (entity, invincibility) in (&entitites, &mut invincibility).join() {
            invincibility.seconds_left -= delta_time.0;
            if invincibility.seconds_left <= 0.0 {
                updater.remove::<Invincibility>(entity);
                updater.remove::<RenderKind>(entity);
                updater.insert(entity, RenderKind::Player);
            }
        }
    }
}
