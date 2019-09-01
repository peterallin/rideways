use std::collections::btree_map::BTreeMap;

use sdl2::image::LoadTexture;
use sdl2::pixels::Color;
use sdl2::render::{Texture, TextureCreator};
use sdl2::video::WindowContext;
use sdl2::EventPump;

use crate::ecs::RenderKind;

type Canvas = sdl2::render::Canvas<sdl2::video::Window>;
type Map<'a> = BTreeMap<RenderKind, (Texture<'a>, (u32, u32))>;

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
    map: Map<'a>,
    canvas: Canvas,
}

impl<'a> Renderer<'a> {
    pub fn new(canvas: Canvas, texture_creator: &'a TextureCreator<WindowContext>) -> Renderer<'a> {
        let map = Map::new();
        let mut renderer = Renderer { map, canvas };
        renderer.load_texture(RenderKind::UFO, "ufo.png", texture_creator);
        renderer.load_texture(RenderKind::Player, "player.png", texture_creator);
        renderer.load_texture(RenderKind::BasicShot, "basic_shot.png", texture_creator);
        renderer
    }

    fn load_texture(
        &mut self,
        kind: RenderKind,
        filename: &str,
        texture_creator: &'a TextureCreator<WindowContext>,
    ) {
        let texture = texture_creator
            .load_texture(filename)
            .expect("Failed to load texture"); // TODO: Improve error message, include the filename
        let query = texture.query();
        let size = (query.width, query.height);
        self.map.insert(kind, (texture, size));
    }

    pub fn render(
        &mut self,
        position: &super::ecs::Position,
        render_kind: &super::ecs::RenderKind,
    ) {
        let render_info = self
            .map
            .get(render_kind)
            .expect("Failed to get render info"); // TODO: Improve error message, include render kind
        let render_size = render_info.1;
        let dest_rect = sdl2::rect::Rect::new(
            position.rect.left() as i32,
            position.rect.top() as i32,
            render_size.0,
            render_size.1,
        );
        self.canvas
            .copy(&render_info.0, None, dest_rect)
            .expect("Unable to copy ufo image"); // TODO: Improve error message, include render kind
    }

    pub fn clear(&mut self) {
        self.canvas.set_draw_color(Color::RGB(0, 0, 0));
        self.canvas.clear();
    }

    pub fn present(&mut self) {
        self.canvas.present();
    }

    pub fn ufo_size(&self) -> (u32, u32) {
        self.map.get(&RenderKind::UFO).expect("No UFO").1
    }

    pub fn player_size(&self) -> (u32, u32) {
        self.map.get(&RenderKind::Player).expect("No player").1
    }

    pub fn basic_shot_size(&self) -> (u32, u32) {
        self.map
            .get(&RenderKind::BasicShot)
            .expect("No basic shot")
            .1
    }
}
