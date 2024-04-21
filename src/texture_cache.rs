use std::collections::HashMap;

use sdl2::video::Window;
use sdl2::video::WindowContext;
use sdl2::render::Canvas;
use sdl2::render::TextureCreator;
use sdl2::render::Texture;

use super::UIError;

type Txc = TextureCreator<WindowContext>;

pub struct TextureCache<'a> {
  cache: HashMap<String,Texture<'a>>,
}

impl<'a> TextureCache<'a> {
  pub fn new() -> Self {
    Self {
      cache:HashMap::new()
    }
  }

  pub fn store(&mut self,k: &str,t: Texture<'a>) {
    self.cache.insert(k.to_string(),t);
  }

  pub fn get(&self,k:&str) -> Option<&Texture> {
    self.cache.get(k)
  }

  pub fn get_mut(&'a mut self,k:&str) -> Option<&mut Texture> {
    self.cache.get_mut(k)
  }
}
