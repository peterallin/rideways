use std::error::Error;

mod control_state;
mod ecs;
mod graphics;
mod rect;

use control_state::ControlState;
use rect::Rect;

fn main() -> Result<(), Box<dyn Error>> {
    let window = graphics::Graphics::make_window("Rideways", (800, 600));
    let texture_creator = window.canvas.texture_creator();
    let mut graphics = graphics::Graphics::new(window, &texture_creator);
    let (mut world, mut dispatcher) = ecs::setup(graphics.renderer);
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
        dispatcher.dispatch(&world);
    }

    Ok(())
}
