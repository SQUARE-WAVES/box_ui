use sdl2::video::Window;
use sdl2::render::Canvas;
use sdl2::render::Texture;
use sdl2::rect::Rect;
use sdl2::rect::Point;

use super::UIError;

pub trait Stamp {
  fn stamp_all<T1>(&self,canv: &mut Canvas<Window>,dest:T1) -> Result<(),UIError>
  where T1:Into<Rect>;

  fn stamp_segment<T1,T2>(&self,canv: &mut Canvas<Window>,src: T1, dest:T2) -> Result<(),UIError>
  where T1: Into<Rect>,T2: Into<Rect>;

  fn stamp_ex<T1,T2,T3>(
    &self, 
    canv: &mut Canvas<Window>,
    src: T1, 
    dest:T2,
    rot:f64,
    center:T3,
    flip_h:bool,
    flip_v:bool
  ) -> Result<(),UIError>
  where T1:Into<Rect>, T2:Into<Rect>, T3:Into<Point>;
}

impl<'a> Stamp for Texture<'a> {
  fn stamp_all<T1>(&self,canv: &mut Canvas<Window>,dest:T1) -> Result<(),UIError>
  where T1:Into<Rect>
  {
    canv.copy(self,None,dest.into()).map_err(UIError::TextureCopy)
  }

  fn stamp_segment<T1,T2>(&self,canv: &mut Canvas<Window>,src: T1, dest:T2) -> Result<(),UIError>
  where T1: Into<Rect>,T2: Into<Rect> {
    canv.copy(self,src.into(),dest.into()).map_err(UIError::TextureCopy)
  }

  fn stamp_ex<T1,T2,T3>(
    &self, 
    canv: &mut Canvas<Window>,
    src: T1, 
    dest:T2,
    rot:f64,
    center:T3,
    flip_h:bool,
    flip_v:bool
  ) -> Result<(),UIError>
  where T1:Into<Rect>, T2:Into<Rect>, T3:Into<Point> 
  {
    canv.copy_ex(
      self,
      src.into(),
      dest.into(),
      rot,
      center.into(),
      flip_h,
      flip_v
    ).
    map_err(UIError::TextureCopy)
  }
}
