extern crate box_ui;

use std::error::Error;
use box_ui::UISystem;
use sdl2::event::Event;

const colors : [(u8,u8,u8,u8);4] = [
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
      _=> true
    }
}

fn main() -> Result<(),Box<dyn Error>> {
  let mut sys = UISystem::new()?;
  let mut scr = sys.new_screen("random rects",250,250)?;
  let mut rng = LFSR::new();

  while sys.handle_events(eventer) {
    scr.start_frame();
    
    scr.one_shot(0,0,250,250,|mut dc,_| {
      for _i in 0..10 {
        let color = colors[rng.get(3) as usize];
        dc.set_color_tup(color);
        let x:i32 = (rng.get(199) + 1) as i32;
        let y:i32 = (rng.get(199) + 1) as i32;
        let w:u32 = rng.get(49) + 1;
        let h:u32 = rng.get(49) + 1;

        dc.fill_rectangle(x,y,w,h);
      }
    });

    scr.end_frame();
  }

  Ok(())
}
