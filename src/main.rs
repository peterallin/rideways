use std::error::Error;

mod control_state;
mod ecs;
mod entity_sizes;
mod game_state;
mod geometry;
mod graphics;

use control_state::ControlState;
use game_state::{GameState, PlayingGameState};
use geometry::Rect;
use graphics::{FontType, Graphics, TextPosition};

use ecs::components::{IsPlayer, Position, RenderKind};
use sdl2::pixels::Color;
use specs::{Dispatcher, Join, Read, ReadStorage, World, WorldExt};

#[derive(Default)]
pub struct Arena(Rect);
#[derive(Default, Debug)]
pub struct ElapsedSeconds(f32);

fn main() -> Result<(), Box<dyn Error>> {
    let window_size = (1200, 600);
    let sdl_contexts = graphics::Contexts::new()?;
    let window = graphics::Graphics::make_window(&sdl_contexts, "Rideways", window_size)?;
    let texture_creator = window.canvas.texture_creator();
    let mut graphics = graphics::Graphics::new(window, &sdl_contexts, &texture_creator)?;
    let (mut world, mut dispatcher) = ecs::setup(graphics.entity_sizes()?)?;
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

        graphics.clear();
        state = match state {
            GameState::Idle { button_pressed } => {
                let state = idle(button_pressed, control_state, &mut graphics)?;
                if let GameState::Playing { .. } = state {
                    ecs::initialize_world(&mut world, graphics.entity_sizes()?)?;
                }
                state
            }
            GameState::Playing { state } => play(
                state,
                &mut world,
                &mut dispatcher,
                control_state,
                delta_time,
                &mut graphics,
            )?,
            GameState::GameOver { seconds_left } => {
                game_over(seconds_left, delta_time, &mut graphics)?
            }
        };
        graphics.present();
    }

    Ok(())
}

fn game_over(
    seconds_left: f64,
    seconds_passed: f64,
    graphics: &mut Graphics,
) -> Result<GameState, Box<dyn Error>> {
    graphics.draw_text(
        "Game Over",
        TextPosition::Center(600, 300),
        Color::RGBA(255, 0, 0, 0),
        FontType::Title,
    )?;

    let new_state = if seconds_left > seconds_passed {
        GameState::GameOver {
            seconds_left: seconds_left - seconds_passed,
        }
    } else {
        GameState::Idle {
            button_pressed: false,
        }
    };
    Ok(new_state)
}

fn idle(
    button_pressed: bool,
    control_state: ControlState,
    graphics: &mut Graphics,
) -> Result<GameState, Box<dyn Error>> {
    graphics.draw_text(
        "Rideways",
        TextPosition::Center(600, 300),
        Color::RGBA(255, 0, 0, 0),
        FontType::Title,
    )?;
    graphics.draw_text(
        "Press fire to play",
        TextPosition::Center(600, 500),
        Color::RGBA(255, 0, 0, 0),
        FontType::Info,
    )?;
    let new_state = if control_state.fire {
        GameState::Idle {
            button_pressed: true,
        }
    } else if button_pressed {
        GameState::Playing {
            state: PlayingGameState::new(),
        }
    } else {
        GameState::Idle { button_pressed }
    };
    Ok(new_state)
}

fn play(
    state: PlayingGameState,
    world: &mut World,
    dispatcher: &mut Dispatcher,
    control_state: ControlState,
    delta_time: f64,
    graphics: &mut Graphics,
) -> Result<GameState, Box<dyn Error>> {
    let score_text = format!("Score: {}", state.score);
    graphics.draw_text(
        &score_text,
        TextPosition::TopRight(1200, 0),
        Color::RGB(255, 255, 255),
        FontType::Info,
    )?;
    world.insert(control_state);
    world.insert(ElapsedSeconds(delta_time as f32));
    world.insert(state);
    world.maintain();

    dispatcher.dispatch(&world);
    type RenderSystemData<'a> = (ReadStorage<'a, Position>, ReadStorage<'a, RenderKind>);
    let (positions, render_kinds): RenderSystemData = world.system_data();
    for (position, render_kind) in (&positions, &render_kinds).join() {
        graphics.draw_sprite(position, render_kind)?;
    }

    let new_state: (Read<'_, PlayingGameState>) = world.system_data();

    let is_player: ReadStorage<IsPlayer> = world.system_data();
    if is_player.is_empty() {
        Ok(GameState::GameOver { seconds_left: 2.0 })
    } else {
        Ok(GameState::Playing { state: *new_state })
    }
}
