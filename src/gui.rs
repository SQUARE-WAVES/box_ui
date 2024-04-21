use std::collections::HashMap;
use sdl2::video::Window;
use sdl2::video::WindowContext;
use sdl2::render::Canvas;
use sdl2::render::TextureCreator;
use sdl2::render::Texture;
use sdl2::event::Event;
use sdl2::pixels::Color;
use sdl2::image::LoadTexture;
use sdl2::surface::Surface;

use super::UIError;
use super::io_state::IOState;
use super::EventDispatcher;
use super::io_context::IOContext;
use super::draw_context::DrawContext;
use super::texture_cache::TextureCache;

pub trait Widget<T> {
  fn bounds(&self) -> (i32,i32,u32,u32);
  fn draw(&mut self, io: IOContext,draw: DrawContext) -> T;
}

pub struct Gui<'a,'b> where 'a:'b {
  state:IOState,
  canv:&'a mut Canvas<Window>,
  tc:&'a mut TextureCreator<WindowContext>,
  textures:HashMap<&'static str,Texture<'b>>
}

impl<'a,'b> Gui<'a,'b>  where 'a:'b {

	pub fn new(canv: &'a mut Canvas<Window>, tc: &'a mut TextureCreator<WindowContext>) -> Result<Self,UIError> {
		Ok(Self {
			state: IOState::new(),
			canv:canv,
      tc:tc,
      textures:HashMap::new()
		})
	}

  pub fn store_surface(& mut self,key:&'static str,s:Surface) {
    let t = s.as_texture(&self.tc).expect("oh no");
    self.textures.insert(key,t);
  }
  
	pub fn start_frame(&mut self) {
			self.canv.set_draw_color(Color::RGBA(0x00,0x00,0x00,0xFF));
			self.canv.clear();
	}
	
	pub fn end_frame(&mut self) {
		self.canv.present();
		self.state.flip();
	}

  pub fn handle_event(&mut self, ev: &Event) -> bool {
    match ev {
      Event::Quit{..} => false,
      _ => self.state.process_event(ev)
    }
  }

	pub fn one_shot<CB,T>(&mut self,x:i32,y:i32,w:u32,h:u32,cb:CB) -> T
  where CB: FnOnce(IOContext,DrawContext) -> T
  {
		let state = &(self.state);
		let canv = &mut (self.canv);
		let txc = &mut (self.textures);
		let io = IOContext::new(state,x,y,w,h);
		let draw = DrawContext::new(canv,txc,x,y,w,h);

		cb(io,draw)
	}

  pub fn draw_widget<RET,T: Widget<RET>> (&'a mut self, widget: &mut T) -> RET {
    let state = &(self.state);
		let canv = &mut (self.canv);
		let txc = &mut (self.textures);
    let (x,y,w,h) = widget.bounds();
		let io = IOContext::new(state,x,y,w,h);
		let draw = DrawContext::new(canv,txc,x,y,w,h);

    widget.draw(io,draw)
  }
}

impl<'a,'b> EventDispatcher for Gui<'a,'b> {
  fn dispatch_event(&mut self,ev:&Event) -> bool {
    self.handle_event(ev)
  }
}
