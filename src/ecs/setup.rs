use std::error::Error;

use crate::ecs::systems::alien_shooting_system::AlienShooting;
use crate::ecs::systems::collision_checker_system::CollisionChecker;
use crate::ecs::systems::enemy_spawning_system::EnemySpawning;
use crate::ecs::systems::force_inside_system::ForceInside;
use crate::ecs::systems::invincibility_watching_system::InvincibilityWatching;
use crate::ecs::systems::lifetime_watching_system::LifetimeWatching;
use crate::ecs::systems::non_player_control_system::NonPlayerControl;
use crate::ecs::systems::player_control_system::PlayerControl;
use crate::ecs::systems::player_shooting_system::PlayerShooting;
use crate::ecs::systems::reap_outsiders_system::ReapOutsiders;
use crate::ecs::systems::spawner_spawning_system::SpawnerSpawning;
use crate::ecs::systems::update_pos_system::UpdatePos;

use crate::ecs::components::{
    HarmsAliens, HarmsPlayer, Invincibility, IsAlien, IsPlayer, KeepInside, Lifetime, MovementKind,
    Position, ReapWhenOutside, RenderKind, SpawnerKind, Velocity,
};
use crate::entity_sizes::EntitySizes;
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
