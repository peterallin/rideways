use std::collections::btree_map::BTreeMap;
use std::error::Error;

use sdl2::image::LoadTexture;
use sdl2::pixels::Color;
use sdl2::render::{BlendMode, Texture, TextureCreator};
use sdl2::ttf::Font;
use sdl2::video::WindowContext;
use sdl2::EventPump;

use crate::ecs::components::RenderKind;
use crate::entity_sizes::EntitySizes;

type Canvas = sdl2::render::Canvas<sdl2::video::Window>;
type Map<'a> = BTreeMap<RenderKind, (Texture<'a>, (u32, u32))>;

pub struct Window {
    pub canvas: Canvas,
    pub event_pump: EventPump,
}

pub struct Contexts {
    sdl: sdl2::Sdl,
    ttf: sdl2::ttf::Sdl2TtfContext,
}

pub enum FontType {
    Title,
    Info,
}

impl Contexts {
    pub fn new() -> Result<Self, Box<dyn Error>> {
        let sdl = sdl2::init()?;
        let ttf = sdl2::ttf::init()?;
        Ok(Self { sdl, ttf })
    }
}

pub struct Graphics<'a> {
    pub event_pump: EventPump,
    renderer: Renderer<'a>,
    texture_creator: &'a TextureCreator<WindowContext>,
    title_font: Font<'a, 'a>,
    info_font: Font<'a, 'a>,
}

impl<'a> Graphics<'a> {
    pub fn new(
        window: Window,
        contexts: &'a Contexts,
        texture_creator: &'a TextureCreator<WindowContext>,
    ) -> Result<Graphics<'a>, Box<dyn Error>> {
        let renderer = Renderer::new(window.canvas, &texture_creator)?;

        // TODO: Find a way to locate font files
        let title_font = contexts
            .ttf
            .load_font("/usr/share/fonts/TTF/OpenSans-ExtraBold.ttf", 100)?;
        let info_font = contexts
            .ttf
            .load_font("/usr/share/fonts/TTF/OpenSans-ExtraBold.ttf", 40)?;
        Ok(Graphics {
            event_pump: window.event_pump,
            renderer,
            texture_creator,
            title_font,
            info_font,
        })
    }

    pub fn make_window(
        contexts: &Contexts,
        name: &str,
        size: (u32, u32),
    ) -> Result<Window, Box<dyn Error>> {
        let video_context = contexts.sdl.video()?;
        let canvas = video_context
            .window(name, size.0, size.1)
            .position_centered()
            .build()?
            .into_canvas()
            .present_vsync()
            .build()?;
        let event_pump = contexts.sdl.event_pump()?;
        Ok(Window { event_pump, canvas })
    }

    pub fn entity_sizes(&self) -> Result<EntitySizes, Box<dyn Error>> {
        self.renderer.entity_sizes()
    }

    pub fn clear(&mut self) {
        self.renderer.canvas.clear();
    }

    pub fn present(&mut self) {
        self.renderer.present();
    }

    pub fn draw_sprite(
        &mut self,
        target: &crate::ecs::components::Position,
        render_kind: &RenderKind,
    ) -> Result<(), Box<dyn Error>> {
        self.renderer.render(target, render_kind)
    }

    pub fn draw_text(
        &mut self,
        text: &str,
        center_position: (u32, u32),
        color: Color,
        font_type: FontType,
    ) -> Result<(), Box<dyn Error>> {
        let font = match font_type {
            FontType::Info => &self.info_font,
            FontType::Title => &self.title_font,
        };
        let texture = self
            .texture_creator
            .create_texture_from_surface(font.render(text).solid(color)?)?;
        let query = texture.query();
        let top_left = (
            center_position.0 - query.width / 2,
            center_position.1 - query.height / 2,
        );
        let rect = sdl2::rect::Rect::new(
            top_left.0 as i32,
            top_left.1 as i32,
            query.width,
            query.height,
        );
        self.renderer.canvas.copy(&texture, None, rect)?;
        Ok(())
    }
}

pub struct Renderer<'a> {
    map: Map<'a>,
    pub canvas: Canvas,
}

impl<'a> Renderer<'a> {
    pub fn new(
        canvas: Canvas,
        texture_creator: &'a TextureCreator<WindowContext>,
    ) -> Result<Renderer<'a>, Box<dyn Error>> {
        let map = Map::new();
        let mut renderer = Renderer { map, canvas };
        renderer.load_texture(
            RenderKind::UFO,
            "ufo.png",
            texture_creator,
            BlendMode::Blend,
        )?;
        renderer.load_texture(
            RenderKind::Player,
            "player.png",
            texture_creator,
            BlendMode::Blend,
        )?;
        renderer.load_texture(
            RenderKind::BasicShot,
            "basic_shot.png",
            texture_creator,
            BlendMode::Blend,
        )?;
        renderer.load_texture(
            RenderKind::UFOShot,
            "ufo_shot.png",
            texture_creator,
            BlendMode::Blend,
        )?;
        renderer.load_texture(
            RenderKind::Glow,
            "glow.png",
            texture_creator,
            BlendMode::Add,
        )?;
        Ok(renderer)
    }

    fn load_texture(
        &mut self,
        kind: RenderKind,
        filename: &str,
        texture_creator: &'a TextureCreator<WindowContext>,
        blend_mode: BlendMode,
    ) -> Result<(), Box<dyn Error>> {
        let mut texture = texture_creator.load_texture(filename)?;
        texture.set_blend_mode(blend_mode);
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

    pub fn present(&mut self) {
        self.canvas.present();
    }

    pub fn entity_sizes(&self) -> Result<EntitySizes, Box<dyn Error>> {
        let ufo_size = self.map.get(&RenderKind::UFO).ok_or("No UFO")?.1;
        let player_size = self.map.get(&RenderKind::Player).ok_or("No player")?.1;
        let basic_shot_size = self
            .map
            .get(&RenderKind::BasicShot)
            .ok_or("No basic shot")?
            .1;
        let ufo_shot_size = self.map.get(&RenderKind::UFOShot).ok_or("No UFO shot")?.1;
        Ok(EntitySizes {
            ufo_size,
            player_size,
            basic_shot_size,
            ufo_shot_size,
        })
    }
}
