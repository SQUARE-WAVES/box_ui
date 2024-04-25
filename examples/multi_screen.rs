use std::error::Error;
use std::collections::HashMap;

use sdl2::event::Event;
use sdl2::event::WindowEvent;

use box_ui::EventDispatcher;
use box_ui::UISystem;
use box_ui::Screen;
use box_ui::DrawContext; //using this for the trait impl
use box_ui::IOContext; //once again the traits!
use box_ui::FrameDrawer;

const COLORS: [(u8,u8,u8,u8);4] = [
  (0xFF,0xFF,0xFF,0xFF),
  (0xFF,0x55,0xFF,0xFF),
  (0x55,0xFF,0xFF,0xFF),
  (0x55,0x55,0x55,0x55)
];

pub struct Drawer {
  color:usize
}

impl<T:DrawContext> FrameDrawer<T,()> for Drawer {
  fn draw_frame(&mut self,cnv:&mut T,io:&IOContext) {
    cnv.set_rgba(0,0,0,0);
    cnv.clear();

    let (x,y) = io.mouse_pos();
    cnv.set_color(COLORS[self.color]);
    cnv.draw_rectangle(x-25,y-25,50,50).expect("oh no the drawing is broken");

    if io.end_left_click(){
      self.color = (self.color+1)%4;
    }

    cnv.present();
  }
}

pub struct Dispatcher {
  map:HashMap<u32,(Screen,Drawer)>
}

impl Dispatcher {
  pub fn map(&mut self) -> &mut HashMap<u32,(Screen,Drawer)> {
    &mut self.map
  }
}

//gotta figure out a better way to do this!
impl EventDispatcher for Dispatcher {
  fn dispatch_event(&mut self,ev: &sdl2::event::Event) -> bool {
    match ev {
      Event::Quit{..} => false,

      Event::Window{window_id:w,win_event:WindowEvent::Close,..} => {
        self.map.remove(&w);
        true
      }

      e @ Event::KeyDown{window_id:w,..} => {
        self.map.get_mut(&w).map(|(s,_d)|s.process_event(e)).unwrap_or(true)
      },

      e @ Event::KeyUp{window_id:w,..}  => {
        self.map.get_mut(&w).map(|(s,_d)|s.process_event(e)).unwrap_or(true)
      },

      e @ Event::MouseButtonDown{window_id:w,..}  => {
        self.map.get_mut(&w).map(|(s,_d)|s.process_event(e)).unwrap_or(true)
      },

      e @ Event::MouseButtonUp{window_id:w,..}  => {
        self.map.get_mut(&w).map(|(s,_d)|s.process_event(e)).unwrap_or(true)
      },

      e @ Event::MouseMotion{window_id:w,..}  => {
        self.map.get_mut(&w).map(|(s,_d)|s.process_event(e)).unwrap_or(true)
      },

      e @ Event::MouseWheel{window_id:w,..}  => {
        self.map.get_mut(&w).map(|(s,_d)|s.process_event(e)).unwrap_or(true)
      },

      _=> true
    }
  }
}

fn main() -> Result<(),Box<dyn Error>> {
  let mut sys = UISystem::new()?;
  let scr1 = sys.new_screen("gondwanaland",250,250)?;
  let scr2 = sys.new_screen("laurasia",250,250)?;

  let d1 = Drawer{color:2};
  let d2 = Drawer{color:3};

  let mut screens : HashMap<u32,(Screen,Drawer)> = HashMap::new();
  
  //now this might be weird if you have textures, cause the 
  //texture creator might borrow some stuff, you might have to 
  //do drawers separate, but in this case I think you can get away with it

  screens.insert(scr1.id(),(scr1,d1));
  screens.insert(scr2.id(),(scr2,d2));

  let mut d = Dispatcher { map:screens };

  while sys.dispatch_events(&mut d) {
    let now = sys.now();
    for (_, (s,d)) in d.map().iter_mut() {
      s.frame(d,now);
    }
  }

  Ok(())
}
