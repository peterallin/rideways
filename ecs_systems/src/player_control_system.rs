use ecs_components::{IsPlayer, Velocity};
use sdl_input::ControlState;
use specs::{Read, ReadStorage, System, WriteStorage};

pub struct PlayerControl;

impl<'a> System<'a> for PlayerControl {
    type SystemData = (
        Read<'a, ControlState>,
        ReadStorage<'a, IsPlayer>,
        WriteStorage<'a, Velocity>,
    );

    fn run(&mut self, (control_state, is_player, mut velocity): Self::SystemData) {
        use specs::Join;
        for (vel, _) in (&mut velocity, &is_player).join() {
            if control_state.left {
                vel.x = -400.0;
            }
            if control_state.right {
                vel.x = 400.0;
            }
            if !control_state.left && !control_state.right {
                vel.x = 0.0;
            }
            if control_state.up {
                vel.y = -400.0;
            }
            if control_state.down {
                vel.y = 400.0;
            }
            if !control_state.up && !control_state.down {
                vel.y = 0.0;
            }
        }
    }
}
