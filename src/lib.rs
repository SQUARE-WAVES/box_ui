//mods
mod ui_error;
mod io_state;
mod screen;
//mod stamp;
mod gui;
mod io_context;
mod draw_context;
mod key_index;
mod key_map;
//mod box_font;
//mod font_writer;
mod texture_cache;

//exports
pub use ui_error::UIError;
pub use screen::Screen;
pub use texture_cache::TextureCache;
//pub use gui::Gui;
pub use io_context::IOContext;
pub use draw_context::DrawContext;
//pub use stamp::Stamp;
//pub use box_font::BoxFont;
//pub use box_font::LetterInfo;

pub trait EventDispatcher {
  fn dispatch_event(&mut self,ev: &sdl2::event::Event) -> bool;
}

pub struct UISystem {
  vid: sdl2::VideoSubsystem,
  evp: sdl2::EventPump,
}

impl UISystem {
  pub fn new() -> Result<UISystem,UIError> {
    let s = sdl2::init().map_err(UIError::SDLInit)?;
    let v = s.video().map_err(UIError::VideoInit)?;

    sdl2::image::init(sdl2::image::InitFlag::PNG).map_err(UIError::ImageInit)?;

    let ev_pump = s.event_pump().map_err(UIError::EventPumpCreation)?;

    Ok(UISystem {
      vid:v,
      evp:ev_pump
    })
  }

  pub fn new_screen(&self,name: &str, w:u32, h:u32) -> Result<Screen,UIError> {
    Screen::new(&self.vid,name,w,h)
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
}
