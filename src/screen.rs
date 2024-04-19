use sdl2::video::Window;
use sdl2::video::WindowContext;
use sdl2::render::Canvas;
use sdl2::render::TextureCreator;

use super::ui_error::UIError;
use super::UISystem;
use super::gui::Gui;
use super::box_font::BoxFont;

pub struct Screen {
  cnv: Canvas<Window>,
  txc: TextureCreator<WindowContext>
}

impl Screen {
  pub fn new(sys: & UISystem, name:& str, w:u32,h:u32) -> Result<Screen,UIError> {
    let wnd = sys.vid.window(name,w,h).
    build().
    map_err(UIError::WindowCreation)?;
    
    let cnv = wnd.
    into_canvas().
    accelerated().
    present_vsync().
    build().
    map_err(UIError::CanvasCreation)?;
    
    let txc = cnv.texture_creator();


    Ok(Screen{cnv:cnv,txc:txc})
  }

  pub fn id(&self) -> u32 { self.cnv.window().id() }

  pub fn create_gui<'a, T:BoxFont>(&'a mut self,fnt: &'a T) -> Result<Gui<T>,UIError> {
    Gui::new(&mut self.cnv,&mut self.txc,fnt)
  }
}
