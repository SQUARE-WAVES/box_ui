//mods
mod ui_error;
mod io_state;
mod screen;
mod io_context;
mod draw_context;
mod box_font;
mod gui;

//exports
pub use sdl2::image::LoadTexture;

pub use ui_error::UIError;
pub use screen::Screen;
pub use screen::FrameDrawer;
pub use draw_context::DrawContext;
pub use io_state::IOState;
pub use io_context::IOContext;
pub use box_font::BoxFont;
pub use box_font::LetterInfo;
pub use gui::Widget;
pub use gui::render_widget;

//not sure about this but maybe it will be ok
pub type Canvas = sdl2::render::Canvas<sdl2::video::Window>;
pub type TextureCreator = sdl2::render::TextureCreator<sdl2::video::WindowContext>;

pub trait EventDispatcher {
  fn dispatch_event(&mut self,ev: &sdl2::event::Event) -> bool;
}

pub struct UISystem {
  vid: sdl2::VideoSubsystem,
  tim: sdl2::TimerSubsystem,
  evp: sdl2::EventPump,
}

impl UISystem {
  pub fn new() -> Result<UISystem,UIError> {
    let s = sdl2::init().map_err(UIError::SDLInit)?;
    let v = s.video().map_err(UIError::VideoInit)?;
    let t = s.timer().map_err(UIError::TimerInit)?;

    sdl2::image::init(sdl2::image::InitFlag::PNG).map_err(UIError::ImageInit)?;

    let ev_pump = s.event_pump().map_err(UIError::EventPumpCreation)?;

    Ok(UISystem {
      vid:v,
      tim:t,
      evp:ev_pump
    })
  }

  pub fn new_screen(&self,name: &str, w:u32, h:u32) -> Result<Screen,UIError> {
    Screen::new(&self.vid,self.tim.performance_frequency(),name,w,h)
  }

  pub fn dispatch_events<D:EventDispatcher>(&mut self,dispatcher: &mut D) -> bool {
    for ev in self.evp.poll_iter() {
      if !dispatcher.dispatch_event(&ev) {
        return false;
      }
    }
    true
  }

  pub fn handle_events<H:FnMut(sdl2::event::Event)-> bool>(&mut self,mut handler: H) -> bool {
    for ev in self.evp.poll_iter() {
      if !handler(ev) {
        return false;
      }
    }
    true
  }

  pub fn now(&self) -> u64 {
    self.tim.performance_counter()
  }
 
  //this is the amount of performance counter ticks per second
  pub fn freq(&self) -> u64 {
    self.tim.performance_frequency()
  }
}
