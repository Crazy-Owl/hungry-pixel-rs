use std::collections::HashMap;
use std::path::PathBuf;

use sdl2::ttf::{Sdl2TtfContext, Font, GlyphMetrics};
use sdl2::render::{Renderer, Texture};
use sdl2::rect::Rect;
use sdl2::pixels::{PixelFormatEnum, Color};
use sdl2::surface::Surface;

const GLYPH_SET: &'static str = "/\\| _-+=abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789!.,'\":;абвгдеёжзийклмнопрстуфхцчшщъыьэюяАБВГДЕЁЖЗИЙКЛМНОПРСТУФХЦЧШЩЪЫЬЭЮЯ";
pub struct FontAtlas {
    texture: Texture,
    glyphs: HashMap<char, Rect>,
    pub metrics: HashMap<char, GlyphMetrics>,
    max_size: (u32, u32),
}

pub struct FontCache {
    context: Sdl2TtfContext,
    pub cache: HashMap<String, FontAtlas>,
}

impl FontCache {
    pub fn new(context: Sdl2TtfContext) -> FontCache {
        FontCache {
            context: context,
            cache: HashMap::new(),
        }
    }

    pub fn load_font<T: Into<String>>(&mut self,
                                      r: &mut Renderer,
                                      key: T,
                                      path: PathBuf,
                                      size: u16) {
        let font = self.context.load_font(path, size).unwrap();
        let font_atlas = Self::generate_font_atlas(r, &font);

        self.cache.insert(key.into(), font_atlas);
    }

    fn get_font_metrics(f: &Font) -> (HashMap<char, GlyphMetrics>, u32, u32) {
        let mut max_x: u32 = 0;
        let mut max_y: u32 = 0;

        (GLYPH_SET.to_string()
             .chars()
             .map(|x| {
                let m = f.find_glyph_metrics(x).unwrap();
                max_x = if m.advance > max_x as i32 {
                    m.advance as u32
                } else {
                    max_x
                };
                max_y = if m.maxy - m.miny > max_y as i32 {
                    (m.maxy - m.miny) as u32
                } else {
                    max_y
                };
                (x, m)
            })
             .collect(),
         max_x,
         max_y)
    }

    pub fn generate_font_atlas(r: &mut Renderer, f: &Font) -> FontAtlas {
        let (metrics, max_x, max_y) = FontCache::get_font_metrics(f);
        r.render_target().unwrap().create_and_set(PixelFormatEnum::RGBA8888, 256, 256).unwrap();
        r.clear();
        let mut glyphs_map: HashMap<char, Rect> = HashMap::new();

        let mut current_x: u32 = 0;
        let mut current_y: u32 = 0;
        for glyph in GLYPH_SET.to_string().chars() {
            if current_x + max_x > 256 {
                current_x = 0;
                current_y += max_y;
            }
            let glyph_surface: Surface<'static> = f.render_char(glyph)
                .solid(Color::RGBA(255, 255, 255, 0))
                .ok()
                .expect("Could not render glyph!");
            let glyph_texture = r.create_texture_from_surface(&glyph_surface).unwrap();
            let glyph_rect: Rect = Rect::new(current_x as i32, current_y as i32, max_x, max_y);
            r.copy(&glyph_texture, None, Some(glyph_rect)).unwrap();
            glyphs_map.insert(glyph, glyph_rect);
            current_x += max_x;
        }

        FontAtlas {
            texture: r.render_target().unwrap().reset().unwrap().unwrap(),
            glyphs: glyphs_map,
            metrics: metrics,
            max_size: (max_x, max_y),
        }
    }

    pub fn render_texture<'a, T: Into<&'a str>>(&self,
                                                r: &mut Renderer,
                                                key: T,
                                                text: &str)
                                                -> Result<Texture, String> {
        let key_str = key.into();
        let font = self.cache.get(key_str).ok_or("Font not found".to_string())?;

        r.render_target()
            .unwrap()
            .create_and_set(PixelFormatEnum::RGBA8888,
                            text.len() as u32 * font.max_size.0,
                            font.max_size.1)
            .unwrap();

        let mut current_x: i32 = 0;

        for character in text.chars() {
            r.copy(&font.texture,
                      font.glyphs.get(&character).map(|ref x| *x.clone()),
                      Some(Rect::new(current_x, 0, font.max_size.0, font.max_size.1)))?;
            current_x += font.max_size.0 as i32;
        }

        r.render_target().unwrap().reset().unwrap().ok_or("Can not render texture!".to_string())
    }

    pub fn render_text<'a, T: Into<&'a str>>(&self,
                                             r: &mut Renderer,
                                             key: T,
                                             text: &str,
                                             x: i32,
                                             y: i32)
                                             -> Result<(), String> {
        let font = self.cache.get(key.into()).ok_or("Font not found".to_string())?;

        let mut current_x: i32 = x;

        for character in text.chars() {
            r.copy(&font.texture,
                      font.glyphs.get(&character).map(|ref x| *x.clone()),
                      Some(Rect::new(current_x, y, font.max_size.0, font.max_size.1)))?;
            current_x += font.max_size.0 as i32;
        }

        Ok(())
    }
}
