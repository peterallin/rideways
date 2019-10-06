use std::error::Error;

mod control_state;
mod ecs;
mod game_state;
mod geometry;
mod graphics;

use control_state::ControlState;
use game_state::GameState;
use geometry::Rect;
use graphics::Graphics;

use ecs::components::{IsPlayer, Position, RenderKind};
use specs::{Dispatcher, Join, ReadStorage, World, WorldExt};

#[derive(Default)]
pub struct Arena(Rect);
#[derive(Default, Debug)]
pub struct ElapsedSeconds(f32);

fn main() -> Result<(), Box<dyn Error>> {
    let window_size = (1200, 600);
    let window = graphics::Graphics::make_window("Rideways", window_size)?;
    let texture_creator = window.canvas.texture_creator();
    let mut graphics = graphics::Graphics::new(window, &texture_creator)?;
    let ttf_context = sdl2::ttf::init()?;
    let font =
        ttf_context.load_font("/usr/share/fonts/liberation/LiberationSans-Regular.ttf", 60)?; // TODO: Find a way to locate font files
    let text_color = sdl2::pixels::Color::RGB(0, 0, 0);
    let rideways_text = texture_creator
        .create_texture_from_surface(font.render("Rideways").blended(text_color)?)?;
    let rideways_text_query = rideways_text.query();
    let (mut world, mut dispatcher) = ecs::setup(&graphics.renderer)?;
    let arena = Arena(Rect::new(
        (0.0, 32.0).into(),
        (window_size.0, window_size.1 - 32).into(),
    ));
    world.insert(arena);
    let mut control_state = ControlState::new();

    let mut state = GameState::new();
    let mut previous_time = time::precise_time_s();
    loop {
        let time = time::precise_time_s();
        let delta_time = time - previous_time;
        previous_time = time;

        let event = graphics.event_pump.poll_event();
        if let Some(event) = event {
            match event {
                sdl2::event::Event::Quit { .. } => break,
                _ => control_state.update(&event),
            }
        }

        state = match state {
            GameState::Idle { button_pressed } => {
                let state = idle(button_pressed, control_state, &mut graphics);
                graphics.renderer.canvas.copy(
                    &rideways_text,
                    None,
                    sdl2::rect::Rect::new(
                        100,
                        100,
                        rideways_text_query.width,
                        rideways_text_query.height,
                    ),
                )?;
                if let GameState::Playing = state {
                    ecs::initialize_world(&mut world, &graphics.renderer)?;
                }
                state
            }
            GameState::Playing => play(
                &mut world,
                &mut dispatcher,
                control_state,
                delta_time,
                &mut graphics,
            )?,
            GameState::GameOver { seconds_left } => {
                game_over(seconds_left, delta_time, &mut graphics)
            }
        };
        graphics.renderer.present();
    }

    Ok(())
}

fn game_over(seconds_left: f64, seconds_passed: f64, graphics: &mut Graphics) -> GameState {
    graphics
        .renderer
        .canvas
        .set_draw_color(sdl2::pixels::Color::RGB(0, 0, 255));
    graphics.renderer.canvas.clear();
    if seconds_left > seconds_passed {
        GameState::GameOver {
            seconds_left: seconds_left - seconds_passed,
        }
    } else {
        GameState::Idle {
            button_pressed: false,
        }
    }
}

fn idle(button_pressed: bool, control_state: ControlState, graphics: &mut Graphics) -> GameState {
    graphics
        .renderer
        .canvas
        .set_draw_color(sdl2::pixels::Color::RGB(255, 0, 0));
    graphics.renderer.canvas.clear();
    if control_state.fire {
        GameState::Idle {
            button_pressed: true,
        }
    } else if button_pressed {
        GameState::Playing
    } else {
        GameState::Idle { button_pressed }
    }
}

fn play(
    world: &mut World,
    dispatcher: &mut Dispatcher,
    control_state: ControlState,
    delta_time: f64,
    graphics: &mut Graphics,
) -> Result<GameState, Box<dyn Error>> {
    world.insert(control_state);
    world.insert(ElapsedSeconds(delta_time as f32));
    world.maintain();

    dispatcher.dispatch(&world);
    graphics.renderer.clear();
    type RenderSystemData<'a> = (ReadStorage<'a, Position>, ReadStorage<'a, RenderKind>);
    let (positions, render_kinds): RenderSystemData = world.system_data();
    for (position, render_kind) in (&positions, &render_kinds).join() {
        graphics.renderer.render(position, render_kind)?;
    }

    let is_player: ReadStorage<IsPlayer> = world.system_data();
    if is_player.is_empty() {
        Ok(GameState::GameOver { seconds_left: 2.0 })
    } else {
        Ok(GameState::Playing)
    }
}
