use std::cmp::min;
use std::cmp::max;

use sdl2::render::Canvas;
use sdl2::render::Texture;
use sdl2::video::Window;
use sdl2::rect::Rect;
use sdl2::rect::Point;
use sdl2::pixels::Color;
use sdl2::render::BlendMode;

use super::TextureCache;
use super::UIError;

type Cnv = Canvas<Window>;

fn clamp<T :Ord>(v: T, minv: T, maxv: T) -> T {
  max(minv,min(v,maxv))
}

pub struct DrawContext<'a,'b> {
	canv : &'a mut Canvas<Window>,
  cache: &'a mut TextureCache<'b>,
	bounds: Rect,
	restore_clip:Option<Rect> 
}

impl<'a,'b> DrawContext<'a,'b> {
	pub fn new(c:&'a mut Cnv,cache:&'a mut TextureCache<'b>,x:i32,y:i32,w:u32,h:u32 ) -> Self {
    let clippy = c.clip_rect();
    Self {
			canv:c,
      cache:cache,
			bounds:(x,y,w,h).into(),
			restore_clip:clippy
		}
	}

  pub fn cut(root: &'a mut Self,x:i32,y:i32,w:u32,h:u32) -> Self {
    let (rx,ry,rw,rh) = root.bounds.into();
    
    //gotta do some clamping here
    let cut_x = clamp(x+rx,0,rx+rw as i32);
    let cut_y = clamp(y+ry,0,ry+rh as i32);
    let cut_w = clamp(w,0,w-x as u32);
    let cut_h = clamp(h,0,h-y as u32);

    let clippy = root.canv.clip_rect();

    Self {
      canv:root.canv,
      cache:root.cache,
      bounds:(cut_x,cut_y,cut_w,cut_h).into(),
      restore_clip:clippy
    }
  }

  pub fn set_color(&mut self,r:u8, g:u8, b:u8, a:u8) {
		self.canv.set_draw_color(Color::RGBA(r,g,b,a));
	}

  pub fn set_color_tup(&mut self,(r,g,b,a):(u8,u8,u8,u8)) {
    self.canv.set_draw_color(Color::RGBA(r,g,b,a));
  }

	pub fn draw_rectangle(&mut self, x: i32,y: i32,w: u32,h: u32) {
		let (off_x, off_y) = self.bounds.top_left().into();

		self.canv.draw_rect(Rect::new(x+off_x,y + off_y,w,h)).expect("draw rectangle failed");
	}
	
	pub fn draw_rect(&mut self,rect:Rect) {
		self.canv.draw_rect(rect).expect("draw rect failed");
	}

	pub fn fill_rectangle(&mut self,x: i32,y: i32,w: u32,h: u32) {
		let (off_x, off_y) = self.bounds.top_left().into();

		self.canv.fill_rect(Rect::new(x+off_x,y + off_y,w,h)).expect("fill rectangle failed");	
	}

	pub fn fill_rect(&mut self,rect:Rect) {
		self.canv.fill_rect(rect).expect("fill rect failed");
	}

	pub fn set_clip(&mut self, x:i32,y:i32,w:u32,h:u32) {
		let (off_x, off_y) = self.bounds.top_left().into();

		self.restore_clip = self.canv.clip_rect();
		let rect = Rect::new(x+off_x,y+off_y,w,h);

		self.canv.set_clip_rect(Some(rect));
	}

	pub fn set_blend(&mut self,mode: BlendMode) {
		self.canv.set_blend_mode(mode);
	}

  //stamp stuff
  pub fn stamp_all<T1>(&mut self,k: &'static str,dest:T1) -> Result<(),UIError>
  where T1:Into<Rect>
  {
    let t = self.cache.get(k).expect("oh no the stamp wasn't found");
    self.canv.copy(t,None,dest.into()).map_err(UIError::TextureCopy)
  }

  pub fn stamp_segment<T1,T2>(&mut self,k: &'static str,src: T1, dest:T2) -> Result<(),UIError>
  where T1: Into<Rect>,T2: Into<Rect> {
    let t = self.cache.get(k).expect("oh no the stamp wasn't found");
    self.canv.copy(t,src.into(),dest.into()).map_err(UIError::TextureCopy)
  }

  pub fn stamp_ex<T1,T2,T3>(
    &mut self, 
    k: &'static str,
    src: T1, 
    dest:T2,
    rot:f64,
    center:T3,
    flip_h:bool,
    flip_v:bool
  ) -> Result<(),UIError>
  where T1:Into<Rect>, T2:Into<Rect>, T3:Into<Option<Point>> 
  {
    let t = self.cache.get(k).expect("oh no the stamp wasn't found");
    self.canv.copy_ex(
      t,
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

impl<'a,'b> Drop for DrawContext<'a,'b> {
	fn drop(&mut self) {
		self.canv.set_clip_rect(self.restore_clip);
		self.canv.set_blend_mode(BlendMode::None);
	}
}
