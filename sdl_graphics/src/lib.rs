use std::collections::btree_map::BTreeMap;
use std::error::Error;

use sdl2::gfx::primitives::DrawRenderer;
use sdl2::image::LoadTexture;
use sdl2::pixels::Color;
use sdl2::render::{BlendMode, Texture, TextureCreator};
use sdl2::ttf::Font;
use sdl2::video::WindowContext;
use sdl2::EventPump;

use ecs_components::Sprite;
use shared_types::EntitySizes;

type Canvas = sdl2::render::Canvas<sdl2::video::Window>;
type Map<'a> = BTreeMap<Sprite, (Texture<'a>, (u32, u32))>;

pub enum TextPosition {
    Center(u32, u32),
    TopRight(u32, u32),
}

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
        let renderer = Renderer::new(window.canvas, texture_creator)?;

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
        self.renderer.canvas.set_draw_color(Color::RGB(0, 0, 0));
        self.renderer.canvas.clear();
    }

    pub fn present(&mut self) {
        self.renderer.present();
    }

    pub fn draw_circle(&mut self, x: f32, y: f32, radius: i16) {
        let _ = self.renderer.canvas.filled_circle(
            x as i16,
            y as i16,
            radius,
            Color::RGB(150, 200, 200),
        );
    }

    // This function is a hack to avoid what I believe to be a bug in SDL2. When
    // on of the SDL2_gfx functions are used before copying textures onto screen,
    // a small artifact from the last color used with the SDL2_gfx functions sometimes
    // appear on the lower right corner of the first texture being copied. By drawing
    // a small black circle, I make this artifact black and thus invisble in this game.
    pub fn back_to_black(&mut self) {
        let _ = self
            .renderer
            .canvas
            .filled_circle(2, 2, 2, Color::RGB(0, 0, 0));
    }

    pub fn draw_sprite(
        &mut self,
        target: &ecs_components::Position,
        sprite: &Sprite,
    ) -> Result<(), Box<dyn Error>> {
        self.renderer.render(target, sprite)
    }

    pub fn draw_text(
        &mut self,
        text: &str,
        text_position: TextPosition,
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
        let top_left = match text_position {
            TextPosition::Center(x, y) => (x - query.width / 2, y - query.height / 2),
            TextPosition::TopRight(x, y) => (x - query.width, y),
        };
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
        renderer.load_texture(Sprite::UFO, "ufo.png", texture_creator, BlendMode::Blend)?;
        renderer.load_texture(
            Sprite::Player,
            "player.png",
            texture_creator,
            BlendMode::Blend,
        )?;
        renderer.load_texture(
            Sprite::PlayerGhost,
            "player_ghost.png",
            texture_creator,
            BlendMode::Blend,
        )?;
        renderer.load_texture(
            Sprite::BasicShot,
            "basic_shot.png",
            texture_creator,
            BlendMode::Blend,
        )?;
        renderer.load_texture(
            Sprite::UFOShot,
            "ufo_shot.png",
            texture_creator,
            BlendMode::Blend,
        )?;
        renderer.load_texture(Sprite::Glow, "glow.png", texture_creator, BlendMode::Add)?;
        Ok(renderer)
    }

    fn load_texture(
        &mut self,
        sprite: Sprite,
        filename: &str,
        texture_creator: &'a TextureCreator<WindowContext>,
        blend_mode: BlendMode,
    ) -> Result<(), Box<dyn Error>> {
        let mut texture = texture_creator.load_texture(filename)?;
        texture.set_blend_mode(blend_mode);
        let query = texture.query();
        let size = (query.width, query.height);
        self.map.insert(sprite, (texture, size));
        Ok(())
    }

    pub fn render(
        &mut self,
        position: &ecs_components::Position,
        sprite: &ecs_components::Sprite,
    ) -> Result<(), Box<dyn Error>> {
        let render_info = self
            .map
            .get(sprite)
            .ok_or_else(|| format!("Failed to get render info for {:?}", sprite))?;
        let texture = &render_info.0;
        let size = render_info.1;
        let dest_rect = sdl2::rect::Rect::new(
            position.rect.left() as i32,
            position.rect.top() as i32,
            size.0,
            size.1,
        );
        self.canvas.copy(texture, None, dest_rect)?;
        Ok(())
    }

    pub fn present(&mut self) {
        self.canvas.present();
    }

    pub fn entity_sizes(&self) -> Result<EntitySizes, Box<dyn Error>> {
        let ufo_size = self.get_texture_size(Sprite::UFO)?;
        let player_size = self.get_texture_size(Sprite::Player)?;
        let basic_shot_size = self.get_texture_size(Sprite::BasicShot)?;
        let ufo_shot_size = self.get_texture_size(Sprite::UFOShot)?;
        Ok(EntitySizes {
            ufo_size,
            player_size,
            basic_shot_size,
            ufo_shot_size,
        })
    }

    fn get_texture_size(&self, sprite: Sprite) -> Result<(u32, u32), String> {
        let render_info = &self
            .map
            .get(&sprite)
            .ok_or(format!("Missing render info for {:?}", sprite))?;
        Ok(render_info.1)
    }
}
