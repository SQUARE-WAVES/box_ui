use std::error::Error;
use std::fmt;

use sdl2::video::WindowBuildError;
use sdl2::IntegerOrSdlError;

#[derive(Debug)]
pub enum UIError {
  TextureLoad(String),
  EventPumpCreation(String),
  SDLInit(String),
  VideoInit(String),
  ImageInit(String),
  WindowCreation(WindowBuildError),
  CanvasCreation(IntegerOrSdlError)
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

      UIError::EventPumpCreation(s) => write!(f,"failed to create event pump: {}",s),

      UIError::SDLInit(s) => write!(f,"SDL initialization failed: {}",s),

      UIError::VideoInit(s) => write!(f,"SDL video subsystem initialization failed: {}",s),

      UIError::ImageInit(s) => write!(f,"SDL Image subsystem initialization failed: {}",s),

      UIError::WindowCreation(e) => e.fmt(f),

      UIError::CanvasCreation(e) => e.fmt(f) 
    }
  }
}
