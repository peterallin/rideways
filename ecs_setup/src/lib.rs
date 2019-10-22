use std::error::Error;

use ecs_systems::AlienShooting;
use ecs_systems::CollisionChecker;
use ecs_systems::EnemySpawning;
use ecs_systems::ForceInside;
use ecs_systems::InvincibilityWatching;
use ecs_systems::LifetimeWatching;
use ecs_systems::NonPlayerControl;
use ecs_systems::PlayerControl;
use ecs_systems::PlayerShooting;
use ecs_systems::ReapOutsiders;
use ecs_systems::SpawnerSpawning;
use ecs_systems::UpdatePos;

use ecs_components::{
    HarmsAliens, HarmsPlayer, Invincibility, IsAlien, IsPlayer, KeepInside, Lifetime, MovementKind,
    Position, ReapWhenOutside, RenderKind, SpawnerKind, Velocity,
};
use shared_types::EntitySizes;
use specs::world::WorldExt;
use specs::{Dispatcher, DispatcherBuilder, World};

pub fn setup<'a>(entity_sizes: EntitySizes) -> Result<(World, Dispatcher<'a, 'a>), Box<dyn Error>> {
    let mut world = World::new();

    world.register::<HarmsAliens>();
    world.register::<HarmsPlayer>();
    world.register::<Invincibility>();
    world.register::<IsAlien>();
    world.register::<IsPlayer>();
    world.register::<KeepInside>();
    world.register::<Lifetime>();
    world.register::<MovementKind>();
    world.register::<Position>();
    world.register::<ReapWhenOutside>();
    world.register::<RenderKind>();
    world.register::<SpawnerKind>();
    world.register::<Velocity>();

    let dispatcher = DispatcherBuilder::new()
        .with(NonPlayerControl, "NonPlayerControl", &[])
        .with(PlayerControl, "PlayerControl", &[])
        .with(
            PlayerShooting::new(entity_sizes.basic_shot_size),
            "PlayerShooting",
            &[],
        )
        .with(
            AlienShooting::new(entity_sizes.ufo_shot_size.into()),
            "AlienShooting",
            &[],
        )
        .with(
            UpdatePos,
            "UpdatePos",
            &["NonPlayerControl", "PlayerControl"],
        )
        .with(ReapOutsiders, "ReapOutsiders", &["UpdatePos"])
        .with(ForceInside, "ForceInside", &["UpdatePos"])
        .with(CollisionChecker, "CollisionChecker", &["ForceInside"])
        .with(
            EnemySpawning::new(entity_sizes.ufo_size.into()),
            "EnemySpawning",
            &[],
        )
        .with(SpawnerSpawning, "SpawnerSpawning", &[])
        .with(LifetimeWatching, "LifetimeWatching", &[])
        .with(InvincibilityWatching, "InvincibilityWatcher", &[])
        .build();

    Ok((world, dispatcher))
}

pub fn initialize_world(world: &mut World) {
    world.delete_all();
}
