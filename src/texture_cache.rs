use std::collections::HashMap;

use sdl2::video::Window;
use sdl2::video::WindowContext;
use sdl2::render::Canvas;
use sdl2::render::TextureCreator;
use sdl2::render::Texture;
use super::UIError;

pub struct TextureCache<'a> {
  tc:&'a TextureCreator<WindowContext>,
  txs:HashMap<&'static str,Texture<'a>>
}

impl<'a> TextureCache<'a> {
  pub fn new(tc: &'a TextureCreator<WindowContext>) -> Self {
    Self {
      tc:tc,
      txs:HashMap::new()
    }
  }

  pub fn load<'b,T>(&mut self, key:&'static str, proc:T) -> Result<(),UIError>
    where 'a:'b, T:FnOnce(&'b TextureCreator<WindowContext>) -> Result<Texture<'a>,UIError>
  {
    match proc(self.tc) {
      Ok(t) => {
        self.txs.insert(key,t);
        Ok(())
      },
      Err(e) => Err(e)
    }
  }

  pub fn get<'b>(&self,key:&'static str) -> Option<&'b Texture> {
    self.txs.get(key)
  }
}
