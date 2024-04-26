use std::error::Error;
use std::fmt;

use sdl2::video::WindowBuildError;
use sdl2::IntegerOrSdlError;
use sdl2::render::TextureValueError;

#[derive(Debug)]
pub enum UIError {
  TextureLoad(String),
  TextureCreate(TextureValueError),
  EventPumpCreation(String),
  SDLInit(String),
  VideoInit(String),
  ImageInit(String),
  TimerInit(String),
  WindowCreation(WindowBuildError),
  CanvasCreation(IntegerOrSdlError),
  LogicalSize(IntegerOrSdlError),
  FrameRate(String),
  TextureCopy(String),
  DrawFailure(String)
}

impl Error for UIError {
  fn source(&self) -> Option<&(dyn Error + 'static)> {
    None
  }
}

impl fmt::Display for UIError {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    match self {
      UIError::TextureLoad(s) => write!(f,"texture load failed: {}",s),
      
      UIError::TextureCreate(e) => e.fmt(f),

      UIError::EventPumpCreation(s) => write!(f,"failed to create event pump: {}",s),

      UIError::SDLInit(s) => write!(f,"SDL initialization failed: {}",s),

      UIError::VideoInit(s) => write!(f,"SDL video subsystem initialization failed: {}",s),

      UIError::ImageInit(s) => write!(f,"SDL image subsystem initialization failed: {}",s),
      
      UIError::TimerInit(s) => write!(f,"SDL timer subsystem initialization failed: {}",s),

      UIError::WindowCreation(e) => e.fmt(f),

      UIError::CanvasCreation(e) => e.fmt(f),

      UIError::FrameRate(s) => write!(f,"setting the framerate failed: {}",s),

      UIError::LogicalSize(e) => e.fmt(f),

      UIError::TextureCopy(s) => write!(f,"Texture copy failed: {}",s),

      UIError::DrawFailure(s) => write!(f,"Drawing failed: {}",s),
    }
  }
}
