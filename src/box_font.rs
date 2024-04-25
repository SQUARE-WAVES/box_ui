use std::cmp::max;

use sdl2::rect::Rect;
use sdl2::render::Texture;

use super::DrawContext;
use super::UIError;

pub struct LetterInfo {
	pub offset:(i32,i32),
	pub src:(i32,i32,u32,u32)
}

impl LetterInfo {
  pub fn src_rect(&self) -> Rect {
    self.src.into()
  }

  pub fn dest_rect(&self, x:i32,y:i32) -> Rect {
    let (off_x,off_y) = self.offset;
    let (_,_,w,h) = self.src;
    
    (off_x+x,off_y+y,w,h).into()
  }
}

pub trait BoxFont {
  fn letter_info(&self, c: &char) -> &LetterInfo;
  fn box_width(&self) -> u32;
  fn box_height(&self) -> u32;
  
  fn bounds(&self, txt: &str) -> (u32,u32) {
    let mut h :u32 = self.box_height();
    let mut w :u32 = 0;
    let mut lc :u32 = 0;

    for letter in txt.chars() {
      match letter {
        '\n' => { 
          h += self.box_height();
          w = max(w,lc);
          lc = 0;
        }
        _ => { lc += self.box_width(); }
      }
    }

    (max(w,lc),h)
  }

  fn write<D>(&self, canv: &mut D,tx:&Texture, x: i32,y: i32,txt: &str) -> Result<(),UIError>
  where D:DrawContext
  {
    let mut xpos = x;
    let mut ypos = y;

    for letter in txt.chars() {
      match letter {
        '\n' => {
          xpos = x;
          ypos += self.box_height() as i32;
        },

        l => {
          let letter = self.letter_info(&l);
          let src_rect = letter.src_rect();
          let dest_rect = letter.dest_rect(xpos,ypos);

          canv.stamp(tx,src_rect,dest_rect)?;
          xpos += self.box_width() as i32;
        }
      };
    }

    Ok(())
  }
}

//=============================================================================
// conveniences
impl From<(i32,i32,i32,i32,u32,u32)> for LetterInfo {
  fn from( (o,o2,s,s2,s3,s4) : (i32,i32,i32,i32,u32,u32) ) -> LetterInfo {
    LetterInfo{
      offset:(o,o2),
      src:(s,s2,s3,s4)
    }
  }
}
