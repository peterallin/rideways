use ecs_components::Lifetime;
use shared_types::ElapsedSeconds;
use specs::{Entities, Read, System, WriteStorage};

pub struct LifetimeWatching;

impl<'a> System<'a> for LifetimeWatching {
    type SystemData = (
        Entities<'a>,
        Read<'a, ElapsedSeconds>,
        WriteStorage<'a, Lifetime>,
    );

    fn run(&mut self, (entities, delta_time, mut lifetime): Self::SystemData) {
        use specs::Join;
        for (entity, mut lifetime) in (&entities, &mut lifetime).join() {
            lifetime.seconds -= delta_time.0;
            if lifetime.seconds <= 0.0 {
                let _ = entities.delete(entity);
            }
        }
    }
}
