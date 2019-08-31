use super::non_player_control_system::NonPlayerControl;
use super::player_control_system::PlayerControl;
use super::player_shooting_system::PlayerShooting;
use super::render_all_system::RenderAll;
use super::update_pos_system::UpdatePos;
use super::{MovementKind, Position, RenderKind, Velocity};
use crate::graphics::Renderer;
use crate::rect::Rect;
use specs::world::WorldExt;
use specs::{Builder, Dispatcher, DispatcherBuilder, World};

pub fn setup<'a>(renderer: Renderer<'a>) -> (World, Dispatcher<'_, '_>) {
    let mut world = World::new();

    world.register::<Position>();
    world.register::<Velocity>();
    world.register::<MovementKind>();
    world.register::<RenderKind>();

    let ufo_size = (renderer.ufo_size.0 as f32, renderer.ufo_size.1 as f32);
    let player_size = (renderer.player_size.0 as f32, renderer.player_size.1 as f32);

    world
        .create_entity()
        .with(Position {
            rect: Rect::new((0.0, 300.0), player_size),
        })
        .with(Velocity { x: 0.0, y: 0.0 })
        .with(RenderKind::Player)
        .build();

    world
        .create_entity()
        .with(Position {
            rect: Rect::new((5.0, 100.0), ufo_size),
        })
        .with(Velocity { x: -3.0, y: 0.0 })
        .with(MovementKind::UFO)
        .with(RenderKind::UFO)
        .build();

    world
        .create_entity()
        .with(Position {
            rect: Rect::new((5.0, 300.0), ufo_size),
        })
        .with(Velocity { x: 1.0, y: 0.0 })
        .with(MovementKind::UFO)
        .with(RenderKind::UFO)
        .build();

    world
        .create_entity()
        .with(Position {
            rect: Rect::new((100.0, 500.0), ufo_size),
        })
        .with(Velocity { x: 10.0, y: -0.1 })
        .with(MovementKind::UFO)
        .with(RenderKind::UFO)
        .build();

    let dispatcher = DispatcherBuilder::new()
        .with(NonPlayerControl, "NonPlayerControl", &[])
        .with(PlayerControl, "PlayerControl", &[])
        .with(PlayerShooting, "PlayerShooting", &[])
        .with(
            UpdatePos,
            "UpdatePos",
            &["NonPlayerControl", "PlayerControl"],
        )
        .with_thread_local(RenderAll { renderer })
        .build();

    (world, dispatcher)
}
