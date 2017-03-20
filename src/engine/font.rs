use std::collections::HashMap;
use std::fmt::{Debug, Formatter, Error};
use std::path::PathBuf;
use std::convert::From;

use sdl2::ttf::{Sdl2TtfContext, Font, GlyphMetrics};
use sdl2::render::{Renderer, Texture};
use sdl2::rect::Rect;
use sdl2::pixels::{PixelFormatEnum, Color};
use sdl2::surface::Surface;

const GLYPH_SET: &'static str = "/\\| _-+=abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789!.,'\":;абвгдеёжзийклмнопрстуфхцчшщъыьэюяАБВГДЕЁЖЗИЙКЛМНОПРСТУФХЦЧШЩЪЫЬЭЮЯ";

#[derive(Debug, Clone, Copy)]
pub struct ColorMod(u8, u8, u8, u8);

impl Default for ColorMod {
    fn default() -> Self {
        ColorMod(255, 255, 255, 255)
    }
}

impl From<(u8, u8, u8, u8)> for ColorMod {
    fn from(arg: (u8, u8, u8, u8)) -> Self {
        ColorMod(arg.0, arg.1, arg.2, arg.3)
    }
}

pub struct RenderableText {
    text: String,
    color_mod: ColorMod,
}

impl RenderableText {
    pub fn new<M: Into<String>>(s: M) -> RenderableText {
        RenderableText {
            text: s.into(),
            color_mod: ColorMod::default(),
        }
    }

    pub fn with_color<M: Into<String>, C: Into<ColorMod>>(s: M, cm: C) -> RenderableText {
        RenderableText {
            text: s.into(),
            color_mod: cm.into(),
        }
    }

    pub fn set_color(&mut self, cm: ColorMod) {
        self.color_mod = cm;
    }
}

impl Into<RenderableText> for String {
    fn into(self) -> RenderableText {
        RenderableText::new(self)
    }
}

impl<'a> From<&'a str> for RenderableText {
    fn from(m: &str) -> RenderableText {
        RenderableText::new(String::from(m))
    }
}

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

    pub fn render_texture<'a, T: Into<&'a str>, M: Into<RenderableText>>(&mut self,
                          r: &mut Renderer,
                          key: T,
                          text: M) -> Result<Texture, String> {
        let key_str = key.into();
        let renderable = text.into();
        {
            let font = self.cache.get_mut(key_str).ok_or_else(|| "Font not found".to_string())?;

            r.render_target()
                .unwrap()
                .create_and_set(PixelFormatEnum::RGBA8888,
                                renderable.text.len() as u32 * font.max_size.0,
                                font.max_size.1)
                .unwrap();
        }

        self.render_text(r, key_str, renderable, 0, 0).unwrap();

        r.render_target()
            .unwrap()
            .reset()
            .unwrap()
            .ok_or_else(|| "Can not render texture!".to_string())
    }

    pub fn render_text<'a, T: Into<&'a str>, M: Into<RenderableText>>(&mut self,
                                                                      r: &mut Renderer,
                                                                      key: T,
                                                                      text: M,
                                                                      x: i32,
                                                                      y: i32)
                                                                      -> Result<(), String> {
        let font = self.cache.get_mut(key.into()).ok_or_else(|| "Font not found".to_string())?;

        let current_color_mod = font.texture.color_mod();
        let current_alpha_mod = font.texture.alpha_mod();
        let renderable = text.into();

        let ColorMod(r_mod, g_mod, b_mod, a_mod) = renderable.color_mod;
        font.texture.set_color_mod(r_mod, g_mod, b_mod);
        font.texture.set_alpha_mod(a_mod);

        let mut current_x: i32 = x;

        for character in renderable.text.chars() {
            r.copy(&font.texture,
                      font.glyphs.get(&character).cloned(),
                      Some(Rect::new(current_x, y, font.max_size.0, font.max_size.1)))?;
            current_x += font.max_size.0 as i32;
        }

        font.texture.set_color_mod(current_color_mod.0,
                                   current_color_mod.1,
                                   current_color_mod.2);
        font.texture.set_alpha_mod(current_alpha_mod);

        Ok(())
    }
}

impl Debug for FontCache {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        write!(f, "FontCache {{ {} fonts }}", self.cache.len())
    }
}
