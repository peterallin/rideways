use sdl2::image::LoadTexture;
use sdl2::pixels::Color;
use sdl2::render::{Texture, TextureCreator};
use sdl2::video::WindowContext;
use sdl2::EventPump;

type Canvas = sdl2::render::Canvas<sdl2::video::Window>;

pub struct Window {
    pub canvas: Canvas,
    pub event_pump: EventPump,
}

pub struct Graphics<'a> {
    pub event_pump: EventPump,
    pub renderer: Renderer<'a>,
}

impl<'a> Graphics<'a> {
    pub fn new(window: Window, texture_creator: &'a TextureCreator<WindowContext>) -> Graphics<'a> {
        let renderer = Renderer::new(window.canvas, &texture_creator);
        Graphics {
            event_pump: window.event_pump,
            renderer,
        }
    }

    pub fn make_window(name: &str, size: (u32, u32)) -> Window {
        let sdl_context = sdl2::init().expect("Failed to init SDL2");
        let video_context = sdl_context.video().expect("Failed to init video subsystem");
        let canvas = video_context
            .window(name, size.0, size.1)
            .position_centered()
            .build()
            .expect("Failed to open window")
            .into_canvas()
            .present_vsync()
            .build()
            .expect("Failed to create canvas");
        let event_pump = sdl_context
            .event_pump()
            .expect("Failed to create event pump");
        Window { event_pump, canvas }
    }
}

pub struct Renderer<'a> {
    ufo_texture: Texture<'a>,
    pub ufo_size: (u32, u32),
    player_texture: Texture<'a>,
    pub player_size: (u32, u32),
    basic_shot_texture: Texture<'a>,
    pub basic_shot_size: (u32, u32),
    canvas: Canvas,
}

impl<'a> Renderer<'a> {
    pub fn new(canvas: Canvas, texture_creator: &'a TextureCreator<WindowContext>) -> Renderer<'a> {
        let ufo_texture = texture_creator
            .load_texture("ufo.png")
            .expect("Unable to load ufo.png"); // TODO: Any good way to fail a constructor?
        let ufo_query = ufo_texture.query();
        let ufo_size = (ufo_query.width, ufo_query.height);

        let player_texture = texture_creator
            .load_texture("player.png")
            .expect("Unable to load player.png");
        let player_query = player_texture.query();
        let player_size = (player_query.width, player_query.height);

        let basic_shot_texture = texture_creator
            .load_texture("basic_shot.png")
            .expect("Unable to load basic_shot.png");
        let basic_shot_query = basic_shot_texture.query();
        let basic_shot_size = (basic_shot_query.width, basic_shot_query.height);

        Renderer {
            canvas,
            ufo_texture,
            ufo_size,
            player_texture,
            player_size,
            basic_shot_texture,
            basic_shot_size,
        }
    }

    pub fn render(
        &mut self,
        position: &super::ecs::Position,
        render_kind: &super::ecs::RenderKind,
    ) {
        match render_kind {
            super::ecs::RenderKind::UFO => {
                let dest_rect = sdl2::rect::Rect::new(
                    position.rect.left() as i32,
                    position.rect.top() as i32,
                    self.ufo_size.0,
                    self.ufo_size.1,
                );
                self.canvas
                    .copy(&self.ufo_texture, None, dest_rect)
                    .expect("Unable to copy ufo image"); // TODO: Any better way to handle this? At least get the error text out.
            }
            super::ecs::RenderKind::Player => {
                // TODO: This must be doable with less copy-paste
                let dest_rect = sdl2::rect::Rect::new(
                    position.rect.left() as i32,
                    position.rect.top() as i32,
                    self.player_size.0,
                    self.player_size.1,
                );
                self.canvas
                    .copy(&self.player_texture, None, dest_rect)
                    .expect("Unable to copy player image"); // TODO: Any better way to handle this? At least get the error text out.
            }
            super::ecs::RenderKind::BasicShot => {
                // TODO: This must be doable with less copy-paste
                let dest_rect = sdl2::rect::Rect::new(
                    position.rect.left() as i32,
                    position.rect.top() as i32,
                    self.basic_shot_size.0,
                    self.basic_shot_size.1,
                );
                self.canvas
                    .copy(&self.basic_shot_texture, None, dest_rect)
                    .expect("Unable to copy basic shot image"); // TODO: Any better way to handle this? At least get the error text out.
            }
        }
    }

    pub fn clear(&mut self) {
        self.canvas.set_draw_color(Color::RGB(0, 0, 0));
        self.canvas.clear();
    }

    pub fn present(&mut self) {
        self.canvas.present();
    }
}
