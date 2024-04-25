/*
use std::cmp::max;
use std::cmp::min;

use super::DrawContext;

pub struct<'a,T:DrawContext> BoundedDraw<'a,T> {
  x:i32,
  y:i32,
  w:u32,
  h:u32,
  draw:&'a mut T
}

impl<'a,T:DrawContext> BoundedDraw<'a,T> {
  pub fn bounds(&self) { (self.x,self.y,self.w,self.h) }
}

impl<'a> DrawContext for BoundedDraw {
  //starting and ending the frame
  fn clear(&mut self) { self.draw.fill_rect(self.bounds()) }
  
  //probably won't use this much
  fn present(&mut self) { self.draw.present() }
  
  //changing colors
  fn set_color<T:Into<Color>>(&mut self,color:T) { self.draw.set_draw_color(color) }
  fn set_rgba<T1:Into<u8>,T2:Into<u8>,T3:Into<u8>,T4:Into<u8>>(&mut self,r:T1,g:T2,b:T3,a:T4) {
    self.draw.set_draw_color(Color::RGBA(r.into(),g.into(),b.into(),a.into()));
  }

  //alpha blend stuff
  fn blend_off(&mut self) { self.draw.set_blend_mode(BlendMode::None) }
  fn blend_on(&mut self) { self.draw.set_blend_mode(BlendMode::Blend) }
  fn blend_add(&mut self) { self.draw.set_blend_mode(BlendMode::Add) }
  fn blend_mod(&mut self) { self.draw.set_blend_mode(BlendMode::Mod) }
  fn blend_mul(&mut self) { self.draw.set_blend_mode(BlendMode::Mul) }

  //rectangles
  fn draw_rect<T:Into<Rect>>(&mut self, r:T) -> Result<(),UIError> {
    let mut base:Rect = r.into();
    base.offset(self.x,self.y);

    self.draw_rect(base).map_err(UIError::DrawFailure)
  }

  fn fill_rect<T:Into<Rect>>(&mut self, r:T) -> Result<(),UIError> {
    let mut base: Rect =r.into();
    base.offset(self.x,self.y);
    self.fill_rect(r.into()).map_err(UIError::DrawFailure)
  }

  //lines
  fn draw_ln<T:Into<Point>,T2:Into<Point>>(&mut self,p1:T,p2:T2)->Result<(),UIError> {

    self.draw_line(p1.offset(self.x,self.y),p2.offset(self.x,self.y).map_err(UIError::DrawFailure)
  }

  //clipping
  fn get_clip(&self) -> Option<(i32,i32,u32,u32)> {
    self.clip_rect().map(|r|(r.x(),r.y(),r.width(),r.height()))
  }

  fn set_clip<T:Into<Rect>>(&mut self,rect:T) {
    self.set_clip_rect(Some(rect.into()));
  }
  
  fn clear_clip(&mut self) {
    self.set_clip_rect(None);
  }

  //textures, for now we will use the concrete sdl2 texture but we might
  //try and abstract this in the future
  fn stamp<T1,T2>(&mut self,txt:&Texture,src:T1,dst:T2) ->Result<(),UIError>
  where 
    T1:Into<Option<Rect>>,
    T2:Into<Option<Rect>>
  {
    self.copy(txt,src,dst).map_err(UIError::TextureCopy)
  }

  fn stamp_ex<T1,T2,T3>(
    &mut self,
    txt:&Texture,
    src: T1, 
    dst:T2,
    rot:f64,
    center:T3,
    flip_h:bool,
    flip_v:bool
  ) -> Result<(),UIError>
  where 
    T1:Into<Option<Rect>>, 
    T2:Into<Option<Rect>>, 
    T3:Into<Option<Point>>
  {
    self.copy_ex(txt,src,dst,rot,center,flip_h,flip_v).map_err(UIError::TextureCopy)
  }
}*/
