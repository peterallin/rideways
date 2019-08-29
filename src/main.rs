mod ecs;
mod graphics;

use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let window = graphics::Graphics::make_window("Rideways", (800, 600));
    let texture_creator = window.canvas.texture_creator();
    let mut graphics = graphics::Graphics::new(window, &texture_creator);
    let (world, mut dispatcher) = ecs::setup(graphics.renderer);

    loop {
        let event = graphics.event_pump.poll_event();
        if let Some(event) = event {
            if let sdl2::event::Event::Quit { .. } = event {
                break;
            }
        }
        dispatcher.dispatch(&world);
    }

    Ok(())
}
