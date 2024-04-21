use sdl2::video::Window;
use sdl2::video::WindowContext;
use sdl2::render::Canvas;
use sdl2::render::TextureCreator;
use sdl2::pixels::Color;

use super::ui_error::UIError;
use super::DrawContext;
use super::TextureCache;

pub struct Screen {
  cnv: Canvas<Window>,
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

    Ok(Self{cnv:cnv,w:w,h:h})
  }

  pub fn id(&self) -> u32 { self.cnv.window().id() }

  pub fn size(&self) -> (u32,u32) {
    (self.w,self.h)
  }

  pub fn texture_creator(&self) -> TextureCreator<WindowContext> {
    self.cnv.texture_creator()
  }

  pub fn start_frame(&mut self) {
	  self.cnv.set_draw_color(Color::RGBA(0x00,0x00,0x00,0xFF));
		self.cnv.clear();
	}
	
	pub fn end_frame(&mut self) {
		self.cnv.present();
	}

	pub fn one_shot<'a,CB,T>(&mut self,txs:&mut TextureCache<'a>,x:i32,y:i32,w:u32,h:u32,cb: CB) -> T
  where CB: FnOnce(DrawContext) -> T
  {
		let cnv = &mut (self.cnv);
		let draw = DrawContext::new(cnv,txs,x,y,w,h);

		cb(draw)
	}
}
