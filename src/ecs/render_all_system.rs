use crate::ecs::{Position, RenderKind};
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
            self.renderer.render(position, render_kind);
        }
        self.renderer.present();
    }
}
