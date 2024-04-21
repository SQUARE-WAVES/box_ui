extern crate sdl2;
extern crate box_ui;

use std::error::Error;

use sdl2::surface::Surface;
use sdl2::pixels::PixelFormatEnum;
use sdl2::pixels::Color;
use sdl2::rect::Point;
use sdl2::rect::Rect;

use box_ui::UISystem;
use box_ui::Gui;
use box_ui::UIError;

//makes a lil triangle gradient guy
fn make_surface() -> Surface<'static> {
  let surf = Surface::new(256,256,PixelFormatEnum::RGBA32).expect("surf create");
  let mut scan = surf.into_canvas().expect("into canv");

  scan.set_draw_color(Color::RGBA(0x00,0x00,0x00,0x00));
  scan.clear();

  for i in 0..255 {
    let mid_start = 128-(i/2) as i32;
    let mid_end = 128+(i/2) as i32;
    let h = i as i32;

    scan.set_draw_color(Color::RGBA(0xFF-i,0x55,0xFF,0xFF));
    scan.draw_line(Point::new(0,h),Point::new(mid_start,h));
    scan.draw_line(Point::new(mid_end,h),Point::new(256,h));

    scan.set_draw_color(Color::RGBA(0x55,0xFF,i,0xFF));
    scan.draw_line(Point::new(mid_start,h),Point::new(mid_end,h));
  }

  scan.into_surface()
}

fn main() -> Result<(),Box<dyn Error>> {
  let mut sys = UISystem::new()?;
  let mut scr = sys.new_screen("borto",600,600)?;
  let mut gui = scr.gui()?;

  let surf = make_surface();

  gui.load_surface("pyramid",surf);

  let mut rot : f64 = 0.0;

  while sys.dispatch_events(&mut gui) {
    gui.start_frame();
    
    gui.one_shot(0,0,250,250,|_io,mut dc| {
      let mut stmp = dc.get_stamp("pyramid").expect("where's the stamp??");
      stmp.copy_ex(None,Some(Rect::new(10,10,256,256)),rot,None,false,false);
    });

    gui.end_frame();
    std::thread::sleep(std::time::Duration::from_millis(100));
    rot = rot + 1.0;
  }
  Ok(())
}
