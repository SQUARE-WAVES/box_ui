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

pub struct DrawContext<'a> {
	canv : &'a mut Canvas<Window>,
	bounds: Rect,
	restore_clip:Option<Rect> 
}

impl<'a> DrawContext<'a> {
	pub fn new(c:&'a mut Cnv,x:i32,y:i32,w:u32,h:u32 ) -> Self {
    let clippy = c.clip_rect();
    Self {
			canv:c,
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
}

impl<'a> Drop for DrawContext<'a> {
	fn drop(&mut self) {
		self.canv.set_clip_rect(self.restore_clip);
		self.canv.set_blend_mode(BlendMode::None);
	}
}
