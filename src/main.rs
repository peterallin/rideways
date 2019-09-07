use std::error::Error;

mod control_state;
mod ecs;
mod graphics;
mod rect;

use control_state::ControlState;
use rect::Rect;

use specs::WorldExt;

#[derive(Default)]
pub struct Arena(Rect);

fn main() -> Result<(), Box<dyn Error>> {
    let window_size = (1200, 600);
    let window = graphics::Graphics::make_window("Rideways", window_size)?;
    let texture_creator = window.canvas.texture_creator();
    let mut graphics = graphics::Graphics::new(window, &texture_creator)?;
    let (mut world, mut dispatcher) = ecs::setup(graphics.renderer)?;
    let arena = Arena(Rect::new(
        (0.0, 32.0),
        (window_size.0 as f32, window_size.1 as f32 - 32.0),
    ));
    world.insert(arena);
    let mut control_state = ControlState::new();

    loop {
        world.insert(control_state);
        let event = graphics.event_pump.poll_event();
        if let Some(event) = event {
            match event {
                sdl2::event::Event::Quit { .. } => break,
                _ => control_state.update(&event),
            }
        }
        world.maintain();
        dispatcher.dispatch(&world);
    }

    Ok(())
}
