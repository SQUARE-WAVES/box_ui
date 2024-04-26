use std::error::Error;
use sdl2::event::Event;

use box_ui::UISystem;
use box_ui::UIError;
use box_ui::DrawContext; //using this for the trait
use box_ui::Canvas;
use box_ui::IOContext;

const COLORS: [(u8,u8,u8,u8);4] = [
  (0xFF,0xFF,0xFF,0xFF),
  (0xFF,0x55,0xFF,0xFF),
  (0x55,0xFF,0xFF,0xFF),
  (0x55,0x55,0x55,0x55)
];

//this is a half assed rng
struct LFSR {
  v:u32
}

impl LFSR{
  pub fn new()->Self{
    Self{v:0x75D352F1}
  }

  pub fn next(&mut self) {
    self.v = self.v ^ self.v<<13;
    self.v = self.v ^ self.v>>17;
    self.v = self.v ^ self.v<<5;
  }

  pub fn get(&mut self,max:u32) -> u32 {
    self.next();
    self.v % max
  }
}

fn eventer(ev: Event) -> bool {
  match ev {
      Event::Quit{..} => false,
      Event::KeyDown{..} => false,
      _=> true
    }
}

fn main() -> Result<(),Box<dyn Error>> {
  let mut sys = UISystem::new()?;
  let mut scr = sys.build_screen("random rects",250,250).build()?;
  let mut rng = LFSR::new();

  let mut draw = |cnv: &mut Canvas,_io:&IOContext| -> Result<(),UIError> {
    cnv.set_rgba(0x00,0x00,0x00,0x00);
    cnv.clear();

    for _i in 0..10 {
      let (r,g,b,a) = COLORS[rng.get(3) as usize];
      let x:i32 = (rng.get(199) + 1) as i32;
      let y:i32 = (rng.get(199) + 1) as i32;
      let w:u32 = rng.get(49) + 1;
      let h:u32 = rng.get(49) + 1;

      cnv.set_rgba(r,g,b,a);
      cnv.fill_rectangle(x,y,w,h)?;

    }

    cnv.present();
    Ok(())
  };

  while sys.handle_events(eventer) {
    let now = sys.now();
    scr.frame(&mut draw,now).expect("oh no drawing the frame broke");

    //this isn't actually a good way to control frame rate
    std::thread::sleep(std::time::Duration::from_millis(50));
  }

  Ok(())
}
