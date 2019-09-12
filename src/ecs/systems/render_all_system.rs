use crate::ecs::components::{Position, RenderKind};
use crate::graphics::Renderer;
use specs::{ReadStorage, System};

pub struct RenderAll<'textures> {
    pub renderer: Renderer<'textures>,
}

impl<'a, 'textures> System<'a> for RenderAll<'textures> {
    type SystemData = (ReadStorage<'a, Position>, ReadStorage<'a, RenderKind>);

    fn run(&mut self, (positions, render_kinds): Self::SystemData) {
        use specs::Join;
        self.renderer.clear();
        for (position, render_kind) in (&positions, &render_kinds).join() {
            match self.renderer.render(position, render_kind) {
                Ok(()) => (),
                Err(e) => eprintln!("Rendering failed: {}", e),
            }
        }
        self.renderer.present();
    }
}
