use sdl2::video::Window;
use sdl2::video::WindowContext;
use sdl2::render::Canvas;
use sdl2::render::TextureCreator;

use super::ui_error::UIError;
use super::texture_cache::TextureCache;
use super::Gui;

pub struct Screen {
  cnv: Canvas<Window>,
  txc: TextureCreator<WindowContext>,
  w:u32,
  h:u32
}

impl Screen {
  pub fn new(vid: &sdl2::VideoSubsystem, name:&str, w:u32,h:u32) -> Result<Self,UIError> {
    let wnd = vid.window(name,w,h).
    build().
    map_err(UIError::WindowCreation)?;
    
    let cnv = wnd.
    into_canvas().
    accelerated().
    present_vsync().
    build().
    map_err(UIError::CanvasCreation)?;

    let txc = cnv.texture_creator();

    Ok(Self{cnv:cnv,txc,w:w,h:h})
  }

  pub fn id(&self) -> u32 { self.cnv.window().id() }

  pub fn size(&self) -> (u32,u32) {
    (self.w,self.h)
  }

  pub fn gui(&mut self) -> Result<Gui,UIError> {
    Gui::new(&mut self.cnv,&mut self.txc)
  }
}
