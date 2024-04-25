use sdl2::video::Window;
use sdl2::video::WindowContext;
use sdl2::render::Canvas;
use sdl2::render::TextureCreator;
use sdl2::event::Event;

use super::ui_error::UIError;
use super::DrawContext;
use super::IOContext;
use super::IOState;

//some convenicnes
use sdl2::VideoSubsystem as Video;

pub trait FrameDrawer<D:DrawContext,T> {
  //the result is so you can use the ? to handle errors in the drawing
  fn draw_frame(&mut self,cnv:&mut D,io: &IOContext)->Result<T,UIError>;
}

pub struct Screen {
  cnv: Canvas<Window>,
  io:IOState
}

impl Screen {
  
  pub fn new(vid: &Video, time_scale:u64, name:&str, w:u32,h:u32) -> Result<Self,UIError> {
    let wnd = vid.window(name,w,h).
    build().
    map_err(UIError::WindowCreation)?;
    
    let cnv = wnd.
    into_canvas().
    accelerated().
    present_vsync().
    build().
    map_err(UIError::CanvasCreation)?;

    Ok(Self{cnv:cnv,io:IOState::new(w,h,time_scale)})
  }

  pub fn id(&self) -> u32 { self.cnv.window().id() }

  pub fn texture_creator(&self) -> TextureCreator<WindowContext> {
    self.cnv.texture_creator()
  }

  pub fn process_event(&mut self,ev:&Event) -> bool {
    self.io.process_event(ev)
  }

	pub fn frame<CB,T>(&mut self,cb: &mut CB,now:u64) -> Result<T,UIError>
  where CB: FrameDrawer<Canvas<Window>,T>
  {
    self.io.current_time = now;
    let (w,h) = self.io.size;
    let ctx = IOContext::new(&self.io,0,0,w,h);

		let cnv = &mut (self.cnv);
		let out = cb.draw_frame(cnv,&ctx);
    self.io.flip();
    out
	}

}

//some generic frame drawing impls
impl<C:DrawContext,R,T:FnMut(&mut C,&IOContext) -> Result<R,UIError>> FrameDrawer<C,R> for T {
  fn draw_frame(&mut self, cnv:&mut C,io:&IOContext) -> Result<R,UIError> {
    self(cnv,io)
  }
}
