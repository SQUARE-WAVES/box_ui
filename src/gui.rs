use sdl2::video::Window;
use sdl2::video::WindowContext;
use sdl2::render::Canvas;
use sdl2::render::TextureCreator;
use sdl2::image::LoadTexture;
use sdl2::event::Event;
use sdl2::pixels::Color;

use super::UIError;
use super::io_state::IOState;
use super::box_font::BoxFont;
use super::font_writer::FontWriter;
use super::EventDispatcher;
use super::io_context::IOContext;
use super::draw_context::DrawContext;

//some type alias
type Cnvs = Canvas<Window>;
type Txtc = TextureCreator<WindowContext>;

pub trait Widget<T,F:BoxFont> {
  fn bounds(&self) -> (i32,i32,u32,u32);
  fn draw(&mut self, io: &IOContext,draw: &mut DrawContext<F>) -> T;
}

pub struct Gui<'a,'b,F:BoxFont> {
  state:IOState,
  canv:&'a mut Canvas<Window>,
  writer:FontWriter<'b,F>
}

impl<'a,'b, F:BoxFont> Gui<'a,'b,F> {

	pub fn new(canv: &'a mut Cnvs,txc: &'b mut Txtc,fnt:&'b F) -> Result<Self,UIError> {
		let path = fnt.asset_path();
		let tx = txc.load_texture(path).map_err(UIError::TextureLoad)?;

		let writer = FontWriter::new(tx,fnt);

		Ok(Gui {
			state: IOState::new(),
			canv:canv,
			writer:writer
		})
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
	where CB: FnOnce(&IOContext,&mut DrawContext<F>) -> T 
  {
		let state = &(self.state);
		let canv = &mut (self.canv);
		let writer = &mut (self.writer);
		let io = IOContext::new(state,x,y,w,h);
		let mut draw = DrawContext::new(canv,writer,x,y,w,h);

		cb(&io,&mut draw)
	}

  pub fn draw_widget<RET,T: Widget<RET,F>> (&mut self, widget: &mut T) -> RET {
    let state = &(self.state);
		let canv = &mut (self.canv);
		let writer = &mut (self.writer);
    let (x,y,w,h) = widget.bounds();
		let io = IOContext::new(state,x,y,w,h);
		let mut draw = DrawContext::new(canv,writer,x,y,w,h);

    widget.draw(&io,&mut draw)
  }
}

impl<'a,'b, F:BoxFont> EventDispatcher for Gui<'a,'b, F> {
  fn dispatch_event(&mut self,ev:&Event) -> bool {
    self.handle_event(ev)
  }
}
