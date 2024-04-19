use sdl2::render::Canvas;
use sdl2::render::Texture;
use sdl2::video::Window;
use sdl2::rect::Rect;

use super::box_font::BoxFont;
use super::box_font::LetterInfo;

pub struct FontWriter<'a,T:BoxFont> {
  txt:Texture<'a>,
  font: &'a T
}

impl<'a,T:BoxFont> FontWriter<'a,T> {
  pub fn new(txt: Texture<'a>, font:&'a T) -> Self {
    Self{txt:txt,font:font}
  }

	pub fn write(&self, canv:&mut Canvas<Window>, x: i32,y: i32,txt: &str) {
    let mut xpos = x;
    let mut ypos = y;

    for letter in txt.chars() {
      match letter {
        '\n' => {
          xpos = x;
          ypos += self.font.box_height() as i32;
        },

        l => {
          let LetterInfo{offset:(o1,o2),src:(s1,s2,s3,s4)} = *self.font.letter_info(&l);
          let src_rect = Rect::new(s1,s2,s3,s4);
          let dest_rect = Rect::new(xpos+o1,ypos+o2,s3,s4);

          //this panics, maybe make it not do that!
          canv.copy(&self.txt,src_rect,dest_rect).expect("texture copy for font failed");
          xpos += self.font.box_width() as i32;
        }
      };
    }
  }

  pub fn set_color(&mut self,r:u8,g:u8,b:u8,a:u8) {
    self.txt.set_color_mod(r,g,b);
    self.txt.set_alpha_mod(a);
  }

  pub fn set_color_tup(&mut self,(r,g,b,a):(u8,u8,u8,u8)) {
    self.set_color(r,g,b,a);
  }
}
