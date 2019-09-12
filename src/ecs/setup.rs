use std::error::Error;

use crate::ecs::systems::collision_checker_system::CollisionChecker;
use crate::ecs::systems::force_inside_system::ForceInside;
use crate::ecs::systems::non_player_control_system::NonPlayerControl;
use crate::ecs::systems::player_control_system::PlayerControl;
use crate::ecs::systems::player_shooting_system::PlayerShooting;
use crate::ecs::systems::reap_outsiders_system::ReapOutsiders;
use crate::ecs::systems::render_all_system::RenderAll;
use crate::ecs::systems::spawning_system::Spawning;
use crate::ecs::systems::update_pos_system::UpdatePos;

use crate::ecs::components::{
    HarmsAliens, IsAlien, KeepInside, MovementKind, Position, ReapWhenOutside, RenderKind, Velocity,
};
use crate::graphics::Renderer;
use crate::rect::Rect;
use specs::world::WorldExt;
use specs::{Builder, Dispatcher, DispatcherBuilder, World};

pub fn setup<'a>(renderer: Renderer<'a>) -> Result<(World, Dispatcher<'_, '_>), Box<dyn Error>> {
    let mut world = World::new();

    world.register::<Position>();
    world.register::<Velocity>();
    world.register::<MovementKind>();
    world.register::<RenderKind>();
    world.register::<ReapWhenOutside>();
    world.register::<KeepInside>();
    world.register::<IsAlien>();
    world.register::<HarmsAliens>();

    let ufo_size = (renderer.ufo_size()?.0 as f32, renderer.ufo_size()?.1 as f32);
    let player_size = (
        renderer.player_size()?.0 as f32,
        renderer.player_size()?.1 as f32,
    );

    world
        .create_entity()
        .with(Position {
            rect: Rect::new((0.0, 300.0), player_size),
        })
        .with(Velocity { x: 0.0, y: 0.0 })
        .with(RenderKind::Player)
        .with(KeepInside)
        .build();

    let dispatcher = DispatcherBuilder::new()
        .with(NonPlayerControl, "NonPlayerControl", &[])
        .with(PlayerControl, "PlayerControl", &[])
        .with(
            PlayerShooting::new(renderer.basic_shot_size()?),
            "PlayerShooting",
            &[],
        )
        // .with(AliensShooting::new
        // )
        .with(
            UpdatePos,
            "UpdatePos",
            &["NonPlayerControl", "PlayerControl"],
        )
        .with(ReapOutsiders, "ReapOutsiders", &["UpdatePos"])
        .with(ForceInside, "ForceInside", &["UpdatePos"])
        .with(CollisionChecker, "CollisionChecker", &["ForceInside"])
        .with(Spawning::new(ufo_size), "Spawning", &[])
        .with_thread_local(RenderAll { renderer })
        .build();

    Ok((world, dispatcher))
}
