use super::non_player_control_system::NonPlayerControl;
use super::render_all_system::RenderAll;
use super::update_pos_system::UpdatePos;
use super::{MovementKind, Position, RenderKind, Velocity};
use crate::graphics::Renderer;
use specs::world::WorldExt;
use specs::{Builder, Dispatcher, DispatcherBuilder, World};

pub fn setup<'a>(renderer: Renderer<'a>) -> (World, Dispatcher<'_, '_>) {
    let mut world = World::new();

    world.register::<Position>();
    world.register::<Velocity>();
    world.register::<MovementKind>();
    world.register::<RenderKind>();

    world
        .create_entity()
        .with(Position { x: 5.0, y: 100.0 })
        .with(Velocity { x: -3.0, y: 0.0 })
        .with(MovementKind::UFO)
        .with(RenderKind::UFO)
        .build();

    world
        .create_entity()
        .with(Position { x: 5.0, y: 300.0 })
        .with(Velocity { x: 1.0, y: 0.0 })
        .with(MovementKind::UFO)
        .with(RenderKind::UFO)
        .build();

    world
        .create_entity()
        .with(Position { x: 100.0, y: 500.0 })
        .with(Velocity { x: 10.0, y: -0.1 })
        .with(MovementKind::UFO)
        .with(RenderKind::UFO)
        .build();

    let dispatcher = DispatcherBuilder::new()
        .with(NonPlayerControl, "NonPlayerControl", &[])
        .with(UpdatePos, "UpdatePos", &["NonPlayerControl"])
        .with_thread_local(RenderAll { renderer })
        .build();

    (world, dispatcher)
}
