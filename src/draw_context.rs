use sdl2::render::Canvas;
use sdl2::render::Texture;
use sdl2::video::Window;
use sdl2::rect::Rect;
use sdl2::rect::Point;
use sdl2::pixels::Color;
use sdl2::render::BlendMode;

use super::UIError;
//---------------------------------------------------------------------------------------
//so the point of this is to abstract out the canvas a bit,
//but a lot of the methods will just be forwards
//the impls are gonna be real tedious, but I can't figure out a better way just yet

pub trait DrawContext {
  //starting and ending the frame
  fn clear(&mut self);
  fn present(&mut self);

  //viewport, for widgets!, I'm using a different name to avoid conflicts
  fn get_portal(&self) -> (i32,i32,u32,u32);
  fn set_portal<T:Into<Rect>>(&mut self, r:T);
  fn clear_portal(&mut self);
  
  //changing colors
  fn set_color<T:Into<Color>>(&mut self,color:T); 
  fn set_rgba<T1:Into<u8>,T2:Into<u8>,T3:Into<u8>,T4:Into<u8>>(&mut self,r:T1,g:T2,b:T3,a:T4);

  //alpha blend stuff
  fn blend_off(&mut self);
  fn blend_on(&mut self);
  fn blend_add(&mut self);
  fn blend_mod(&mut self);
  fn blend_mul(&mut self);

  //rectangles
  fn draw_rect<T:Into<Rect>>(&mut self, r:T) -> Result<(),UIError>;
  fn draw_rectangle(&mut self, x:i32,y:i32,w:u32,h:u32) -> Result<(),UIError> {
    self.draw_rect((x,y,w,h))
  }

  fn fill_rect<T:Into<Rect>>(&mut self, r:T) -> Result<(),UIError>;
  fn fill_rectangle(&mut self, x:i32,y:i32,w:u32,h:u32) -> Result<(),UIError> {
    self.fill_rect((x,y,w,h))
  }

  //lines
  fn draw_ln<T:Into<Point>,T2:Into<Point>>(&mut self,p1:T,p2:T2)->Result<(),UIError>;
  fn draw_line(&mut self,x1:i32,y1:i32,x2:i32,y2:i32)->Result<(),UIError> {
    self.draw_ln((x1,y1),(x2,y2))
  }

  //clipping, we are begnning to make an effort to get away from the sdl rect
  fn get_clip(&self) -> Option<(i32,i32,u32,u32)>;
  fn set_clip<T:Into<Rect>>(&mut self,rect:T);
  
  fn set_clip_rectange(&mut self,x:i32,y:i32,w:u32,h:u32) {
    self.set_clip((x,y,w,h))
  }

  fn clear_clip(&mut self);

  //textures, for now we will use the concrete sdl2 texture but we might
  //try and abstract this in the future

  fn stamp<T1,T2>(&mut self,txt:&Texture,src:T1,dst:T2) ->Result<(),UIError>
  where 
    T1:Into<Option<Rect>>,
    T2:Into<Option<Rect>>;

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
  where T1:Into<Option<Rect>>, T2:Into<Option<Rect>>, T3:Into<Option<Point>>;
}

impl<'a> DrawContext for Canvas<Window> {
  //starting and ending the frame
  fn clear(&mut self) { self.clear() }
  fn present(&mut self) { self.present() }

  //viewport (for widgets!)
  fn get_portal(&self) -> (i32,i32,u32,u32) {
    self.viewport().into()
  }

  fn set_portal<T:Into<Rect>>(&mut self,r:T) {
    self.set_viewport(Some(r.into()));
  }

  fn clear_portal(&mut self) {
    self.set_viewport(None);
  }
  
  //changing colors
  fn set_color<T:Into<Color>>(&mut self,color:T) { self.set_draw_color(color) }
  fn set_rgba<T1:Into<u8>,T2:Into<u8>,T3:Into<u8>,T4:Into<u8>>(&mut self,r:T1,g:T2,b:T3,a:T4) {
    self.set_draw_color(Color::RGBA(r.into(),g.into(),b.into(),a.into()));
  }

  //alpha blend stuff
  fn blend_off(&mut self) { self.set_blend_mode(BlendMode::None) }
  fn blend_on(&mut self) { self.set_blend_mode(BlendMode::Blend) }
  fn blend_add(&mut self) { self.set_blend_mode(BlendMode::Add) }
  fn blend_mod(&mut self) { self.set_blend_mode(BlendMode::Mod) }
  fn blend_mul(&mut self) { self.set_blend_mode(BlendMode::Mul) }

  //rectangles
  fn draw_rect<T:Into<Rect>>(&mut self, r:T) -> Result<(),UIError> {
    self.draw_rect(r.into()).map_err(UIError::DrawFailure)
  }

  fn fill_rect<T:Into<Rect>>(&mut self, r:T) -> Result<(),UIError> {
    self.fill_rect(r.into()).map_err(UIError::DrawFailure)
  }

  //lines
  fn draw_ln<T:Into<Point>,T2:Into<Point>>(&mut self,p1:T,p2:T2)->Result<(),UIError> {
    self.draw_line(p1,p2).map_err(UIError::DrawFailure)
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
}
