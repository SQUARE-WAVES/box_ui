use std::error::Error;

use sdl2::event::Event;

use box_ui::UISystem;
use box_ui::DrawContext; //using this for the trait impl
use box_ui::IOContext; //once again the traits!
use box_ui::Canvas;

const COLORS: [(u8,u8,u8,u8);4] = [
  (0xFF,0xFF,0xFF,0xFF),
  (0xFF,0x55,0xFF,0xFF),
  (0x55,0xFF,0xFF,0xFF),
  (0x55,0x55,0x55,0x55)
];

fn main() -> Result<(),Box<dyn Error>> {
  let mut sys = UISystem::new()?;
  let mut scr = sys.new_screen("pangea",250,250)?;

  let mut color = 0;

  let mut draw = |cnv: &mut Canvas,io:&IOContext| {
    cnv.set_rgba(0x00,0x00,0x00,0x00);
    cnv.clear();

    let (x,y) = io.mouse_pos();
    cnv.set_color(COLORS[color]);
    cnv.draw_rectangle(x-25,y-25,50,50).expect("oh no the drawing is broken");

    if io.end_left_click(){
      color = (color+1)%4;
    }
    

    cnv.present();
  };

  while sys.handle_events(|ev|{
    match ev {
      Event::Quit{..} => false,
      e=>scr.process_event(&e)
    }
  }) {
    scr.frame(&mut draw,sys.now());
  }

  Ok(())
}
