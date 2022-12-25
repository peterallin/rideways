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
use ecs_systems::StarSpawner;
use ecs_systems::UpdatePos;

use ecs_components::{
    Draw, HarmsAliens, HarmsPlayer, Invincibility, IsAlien, IsExplosion, IsPlayer, KeepInside,
    Lifetime, MovementKind, Position, ReapWhenOutside, SpawnerKind, Sprite, Velocity,
};
use shared_types::{ElapsedSeconds, EntitySizes, PlayingGameState};
use specs::world::WorldExt;
use specs::{Dispatcher, DispatcherBuilder, RunNow, World};

pub fn setup<'a>(entity_sizes: EntitySizes) -> Result<(World, Dispatcher<'a, 'a>), Box<dyn Error>> {
    let mut world = World::new();

    world.register::<Draw>();
    world.register::<HarmsAliens>();
    world.register::<HarmsPlayer>();
    world.register::<Invincibility>();
    world.register::<IsAlien>();
    world.register::<IsExplosion>();
    world.register::<IsPlayer>();
    world.register::<KeepInside>();
    world.register::<Lifetime>();
    world.register::<MovementKind>();
    world.register::<Position>();
    world.register::<ReapWhenOutside>();
    world.register::<SpawnerKind>();
    world.register::<Sprite>();
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
        .with(StarSpawner, "StarSpawner", &[])
        .build();

    Ok((world, dispatcher))
}

pub fn initialize_world(world: &mut World) {
    world.delete_all();

    // Add initial stars. The elapsed time of 0.016
    // is close to what we get when the game is running.
    // 3000 iterations is enough to get stars all over
    // the screen (found by fiddling)
    let mut star_spawner = StarSpawner;
    let mut update_position = UpdatePos;
    for _ in 0..3000 {
        world.insert(ElapsedSeconds(0.016));
        world.insert(PlayingGameState::new());
        star_spawner.run_now(world);
        update_position.run_now(world);
    }
}
