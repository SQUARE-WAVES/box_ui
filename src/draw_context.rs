use std::cmp::min;
use std::cmp::max;
use std::collections::HashMap;

use sdl2::render::Canvas;
use sdl2::video::Window;
use sdl2::rect::Rect;
use sdl2::rect::Point;
use sdl2::pixels::Color;
use sdl2::render::BlendMode;
use sdl2::render::Texture;

use super::texture_cache::TextureCache;

type Cnv = Canvas<Window>;

fn clamp<T :Ord>(v: T, minv: T, maxv: T) -> T {
  max(minv,min(v,maxv))
}

pub struct DrawContext<'a,'b> {
	canv : &'a mut Canvas<Window>,
  textures: &'a mut HashMap<&'static str,Texture<'b>>,
	bounds: Rect,
	restore_clip:Option<Rect> 
}

pub struct Stamp<'a,'b> {
  canv: &'a mut Canvas<Window>,
  txt:&'a Texture<'b>,
  bounds:Rect
}

impl<'a,'b> DrawContext<'a,'b>  where 'b:'a {
	pub fn new(c:&'a mut Cnv,txc: &'a mut HashMap<&str,Texture<'b>>,x:i32,y:i32,w:u32,h:u32 ) -> Self {
    let clippy = c.clip_rect();
    Self {
			canv:c,
      textures:txc,
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
      textures:root.textures,
      bounds:(cut_x,cut_y,cut_w,cut_h).into(),
      restore_clip:clippy
    }
  }

  pub fn get_stamp(&mut self, txt_key:&str) -> Option<Stamp> {
    
    match self.textures.get(txt_key) {
      Some(t) => { 
        Some(Stamp{
          canv:& mut self.canv,
          txt:t,
          bounds:self.bounds.into()
        })
      },

      None => None
    }
  }

	pub fn set_color(&mut self,r:u8, g:u8, b:u8, a:u8) {
		self.canv.set_draw_color(Color::RGBA(r,g,b,a));
	}

  pub fn set_color_tup(&mut self,(r,g,b,a):(u8,u8,u8,u8)) {
    self.canv.set_draw_color(Color::RGBA(r,g,b,a));
  }

  //Basic Boxes and lines

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
}

impl<'a,'b> Drop for DrawContext<'a,'b> {
	fn drop(&mut self) {
		self.canv.set_clip_rect(self.restore_clip);
		self.canv.set_blend_mode(BlendMode::None);
	}
}

impl<'a,'b> Stamp<'a,'b> {
  pub fn copy<T1,T2> (&mut self, src:T1, dst:T2)
  where 
    T1:Into<Option<Rect>>,
    T2:Into<Option<Rect>>
  {
    let final_destination = dst.into().
    map(|r| {
      r.left_shifted(self.bounds.x()).bottom_shifted(self.bounds.y())
    }).
    unwrap_or(self.bounds);

    self.canv.copy(self.txt,src,final_destination).expect("texture copy failed");
  }

  pub fn copy_ex<T1,T2,P> (&mut self, src:T1, dst:T2,angle:f64,center:P,flip_h:bool,flip_v:bool)
  where 
    T1:Into<Option<Rect>>,
    T2:Into<Option<Rect>>,
    P:Into<Option<Point>>,
  {
    let final_destination = dst.into().
    map(|r| {
      r.left_shifted(self.bounds.x()).bottom_shifted(self.bounds.y())
    }).
    unwrap_or(self.bounds);

    self.canv.copy_ex(self.txt,src,final_destination,angle,center,flip_h,flip_v).
    expect("texture copy failed");
  }
}
