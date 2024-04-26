use sdl2::video::WindowBuilder;

use super::Screen;
use super::IOState;
use super::UIError;

pub struct ScreenBuilder {
  wb:WindowBuilder,
  w:u32,
  h:u32,
  tim:u64
}

impl ScreenBuilder {

  pub fn new(vid:&sdl2::VideoSubsystem,tim:u64,name:&str,w:u32,h:u32) -> Self {
    Self {
      wb:vid.window(name,w,h),
      w:w,
      h:w,
      tim:tim
    }
  }

  pub fn build(&mut self) -> Result<Screen,UIError> {
    let wnd = self.wb.build().map_err(UIError::WindowCreation)?;
    let io = IOState::new(self.w,self.h,self.tim);
    Screen::new(wnd,io)
  }

  pub fn position_centered(&mut self) -> &mut Self { 
    self.wb.position_centered();
    self
  }

  pub fn fullscreen(&mut self) -> &mut Self { 
    self.wb.fullscreen();
    self
  }

  pub fn fullscreen_desktop(&mut self) -> &mut Self { 
    self.wb.fullscreen_desktop();
    self
  }

  pub fn hidden(&mut self) -> &mut Self { 
    self.wb.hidden();
    self
  }

  pub fn borderless(&mut self) -> &mut Self { 
    self.wb.borderless();
    self
  }

  pub fn resizable(&mut self) -> &mut Self { 
    self.wb.resizable();
    self
  }

  pub fn minimized(&mut self) -> &mut Self { 
    self.wb.minimized();
    self
  }

  pub fn maximized(&mut self) -> &mut Self { 
    self.wb.maximized();
    self
  }

  pub fn input_grabbed(&mut self) -> &mut Self { 
    self.wb.input_grabbed();
    self
  }




}
