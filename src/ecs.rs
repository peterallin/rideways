use super::graphics::Renderer;
use specs::world::WorldExt;
use specs::{
    Builder, Component, Dispatcher, DispatcherBuilder, ReadStorage, System, VecStorage, World,
    WriteStorage,
};
use specs_derive::Component;

#[derive(Debug, Component)]
#[storage(VecStorage)]
pub struct Position {
    pub x: f32,
    pub y: f32,
}

#[derive(Debug, Component)]
#[storage(VecStorage)]
struct Velocity {
    x: f32,
    y: f32,
}

#[derive(Debug, Component)]
#[storage(VecStorage)]
enum MovementKind {
    UFO,
}

#[derive(Debug, Component)]
#[storage(VecStorage)]
pub enum RenderKind {
    UFO,
}

struct NonPlayerControl;

fn control_ufo(position: &Position, velocity: &mut Velocity) {
    // TODO: Get the bounding rect from Specs, and have a rect for the position
    if position.x <= 0.0 || position.x > 800.0 {
        velocity.x = -velocity.x;
    }
}

impl<'a> System<'a> for NonPlayerControl {
    type SystemData = (
        ReadStorage<'a, MovementKind>,
        ReadStorage<'a, Position>,
        WriteStorage<'a, Velocity>,
    );

    fn run(&mut self, (kind, pos, mut vel): Self::SystemData) {
        use specs::Join;
        for (kind, pos, vel) in (&kind, &pos, &mut vel).join() {
            match kind {
                MovementKind::UFO => control_ufo(pos, vel),
            }
        }
    }
}

struct UpdatePos;

impl<'a> System<'a> for UpdatePos {
    type SystemData = (ReadStorage<'a, Velocity>, WriteStorage<'a, Position>);

    fn run(&mut self, (velocity, mut position): Self::SystemData) {
        use specs::Join;
        for (v, p) in (&velocity, &mut position).join() {
            p.x += v.x;
            p.y += v.y;
        }
    }
}

struct RenderAll<'textures> {
    renderer: Renderer<'textures>,
}

impl<'a, 'textures> System<'a> for RenderAll<'textures> {
    type SystemData = (ReadStorage<'a, Position>, ReadStorage<'a, RenderKind>);

    fn run(&mut self, (positions, render_kinds): Self::SystemData) {
        use specs::Join;
        self.renderer.clear();
        for (position, render_kind) in (&positions, &render_kinds).join() {
            self.renderer.render(position, render_kind);
        }
        self.renderer.present();
    }
}

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
