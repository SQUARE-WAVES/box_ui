//mods
mod ui_error;
mod io_state;
mod screen;
mod screen_builder;
mod io_context;
mod draw_context;
mod box_font;
mod gui;

use screen_builder::ScreenBuilder;
use sdl2::gfx::framerate::FPSManager;

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
  frame_buddy:FPSManager,
}

impl UISystem {
  pub fn new() -> Result<UISystem,UIError> {
    let s = sdl2::init().map_err(UIError::SDLInit)?;
    let v = s.video().map_err(UIError::VideoInit)?;
    let t = s.timer().map_err(UIError::TimerInit)?;

    sdl2::image::init(sdl2::image::InitFlag::PNG).map_err(UIError::ImageInit)?;

    let ev_pump = s.event_pump().map_err(UIError::EventPumpCreation)?;
    let mut fps = FPSManager::new();
    fps.set_framerate(60).map_err(UIError::FrameRate)?;

    Ok(UISystem {
      vid:v,
      tim:t,
      evp:ev_pump,
      frame_buddy:fps
    })
  }

  pub fn new_screen(&self,name: &str, w:u32, h:u32) -> Result<Screen,UIError> {
    let wnd = self.vid.window(name,w,h).
    build().
    map_err(UIError::WindowCreation)?;

    let io = IOState::new(w,h,self.tim.performance_frequency());

    Screen::new(wnd,io)
  }

  pub fn build_screen(&self,name: &str, w:u32, h:u32) -> ScreenBuilder {
    ScreenBuilder::new(&self.vid,self.tim.performance_frequency(),name,w,h)
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

  pub fn set_framerate(&mut self, new_fr: u32) -> Result<(),UIError> {
    self.frame_buddy.set_framerate(new_fr).map_err(UIError::FrameRate)
  }

  pub fn frame_delay(&mut self) {
    self.frame_buddy.delay();
  }

  pub fn now(&self) -> u64 {
    self.tim.performance_counter()
  }
 
  //this is the amount of performance counter ticks per second
  pub fn freq(&self) -> u64 {
    self.tim.performance_frequency()
  }
}
