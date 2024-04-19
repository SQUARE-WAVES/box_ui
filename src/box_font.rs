use std::cmp::max;

pub struct LetterInfo {
	pub offset:(i32,i32),
	pub src:(i32,i32,u32,u32)
}

pub trait BoxFont {
  fn asset_path(&self) -> &str;
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
}
