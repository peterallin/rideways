use std::collections::btree_map::BTreeMap;
use std::error::Error;

use sdl2::image::LoadTexture;
use sdl2::pixels::Color;
use sdl2::render::{Texture, TextureCreator};
use sdl2::video::WindowContext;
use sdl2::EventPump;

use crate::ecs::components::RenderKind;

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
    pub fn new(
        window: Window,
        texture_creator: &'a TextureCreator<WindowContext>,
    ) -> Result<Graphics<'a>, Box<dyn Error>> {
        let renderer = Renderer::new(window.canvas, &texture_creator)?;
        Ok(Graphics {
            event_pump: window.event_pump,
            renderer,
        })
    }

    pub fn make_window(name: &str, size: (u32, u32)) -> Result<Window, Box<dyn Error>> {
        let sdl_context = sdl2::init()?;
        let video_context = sdl_context.video()?;
        let canvas = video_context
            .window(name, size.0, size.1)
            .position_centered()
            .build()?
            .into_canvas()
            .present_vsync()
            .build()?;
        let event_pump = sdl_context.event_pump()?;
        Ok(Window { event_pump, canvas })
    }
}

pub struct Renderer<'a> {
    map: Map<'a>,
    canvas: Canvas,
}

impl<'a> Renderer<'a> {
    pub fn new(
        canvas: Canvas,
        texture_creator: &'a TextureCreator<WindowContext>,
    ) -> Result<Renderer<'a>, Box<dyn Error>> {
        let map = Map::new();
        let mut renderer = Renderer { map, canvas };
        renderer.load_texture(RenderKind::UFO, "ufo.png", texture_creator)?;
        renderer.load_texture(RenderKind::Player, "player.png", texture_creator)?;
        renderer.load_texture(RenderKind::BasicShot, "basic_shot.png", texture_creator)?;
        renderer.load_texture(RenderKind::UFOShot, "ufo_shot.png", texture_creator)?;
        Ok(renderer)
    }

    fn load_texture(
        &mut self,
        kind: RenderKind,
        filename: &str,
        texture_creator: &'a TextureCreator<WindowContext>,
    ) -> Result<(), Box<dyn Error>> {
        let texture = texture_creator.load_texture(filename)?;
        let query = texture.query();
        let size = (query.width, query.height);
        self.map.insert(kind, (texture, size));
        Ok(())
    }

    pub fn render(
        &mut self,
        position: &crate::ecs::components::Position,
        render_kind: &crate::ecs::components::RenderKind,
    ) -> Result<(), Box<dyn Error>> {
        let render_info = self
            .map
            .get(render_kind)
            .ok_or_else(|| format!("Failed to get render info for {:?}", render_kind))?;
        let render_size = render_info.1;
        let dest_rect = sdl2::rect::Rect::new(
            position.rect.left() as i32,
            position.rect.top() as i32,
            render_size.0,
            render_size.1,
        );
        self.canvas.copy(&render_info.0, None, dest_rect)?;
        Ok(())
    }

    pub fn clear(&mut self) {
        self.canvas.set_draw_color(Color::RGB(0, 0, 0));
        self.canvas.clear();
    }

    pub fn present(&mut self) {
        self.canvas.present();
    }

    pub fn ufo_size(&self) -> Result<(u32, u32), Box<dyn Error>> {
        Ok(self.map.get(&RenderKind::UFO).ok_or("No UFO")?.1)
    }

    pub fn player_size(&self) -> Result<(u32, u32), Box<dyn Error>> {
        Ok(self.map.get(&RenderKind::Player).ok_or("No player")?.1)
    }

    pub fn basic_shot_size(&self) -> Result<(u32, u32), Box<dyn Error>> {
        Ok(self
            .map
            .get(&RenderKind::BasicShot)
            .ok_or("No basic shot")?
            .1)
    }

    pub fn ufo_shot_size(&self) -> Result<(u32, u32), Box<dyn Error>> {
        Ok(self.map.get(&RenderKind::UFOShot).ok_or("No UFO shot")?.1)
    }
}
