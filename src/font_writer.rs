use super::Stamp;
use super::box_font::BoxFont;

pub fn write<T:BoxFont>(fnt: &T, stamp:&mut Stamp, x: i32,y: i32,txt: &str) {
  let mut xpos = x;
  let mut ypos = y;

  for letter in txt.chars() {
    match letter {
      '\n' => {
        xpos = x;
        ypos += fnt.box_height() as i32;
      },

      l => {
        let letter = fnt.letter_info(&l);
        let src_rect = letter.src_rect();
        let dest_rect = letter.dest_rect(xpos,ypos);

        //this panics, maybe make it not do that!
        stamp.copy(src_rect,dest_rect);
        xpos += fnt.box_width() as i32;
      }
    };
  }
}
