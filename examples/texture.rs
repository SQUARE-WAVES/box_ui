extern crate sdl2;
extern crate box_ui;

use std::error::Error;

use sdl2::surface::Surface;
use sdl2::pixels::PixelFormatEnum;
use sdl2::pixels::Color;
use sdl2::rect::Point;
use sdl2::rect::Rect;
use sdl2::event::Event;
use sdl2::video::WindowContext;
use sdl2::render::TextureCreator;
use sdl2::render::Texture;

use box_ui::UISystem;
use box_ui::DrawContext;
use box_ui::FrameDrawer;
use box_ui::IOContext;

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
    scan.draw_line(Point::new(0,h),Point::new(mid_start,h)).expect("surface line broke");
    scan.draw_line(Point::new(mid_end,h),Point::new(256,h)).expect("surface line 2 broke");

    scan.set_draw_color(Color::RGBA(0x55,0xFF,i,0xFF));
    scan.draw_line(Point::new(mid_start,h),Point::new(mid_end,h)).expect("surface line 3 broke");
  }

  scan.into_surface()
}

struct Gui<'a> {
  tx:Texture<'a>,
  rot:f64
}

impl<'a> Gui<'a> {
  pub fn new(tc: &'a TextureCreator<WindowContext>) -> Self {
    let surf = make_surface();
    let tx = surf.as_texture(tc).expect("texture blew up");
    Self{tx:tx,rot:0.0}
  }
}

impl<'a,T:DrawContext> FrameDrawer<T,()> for Gui<'a> {
  fn draw_frame(&mut self,cnv: &mut T,io:&IOContext) {
    let (_,_,w,h) = io.bounds();
    cnv.set_rgba(0,0,0,0);
    cnv.clear();
    let x = ((w-256)/2) as i32;
    let y = ((h-256)/2) as i32;
    cnv.stamp_ex(&self.tx,None,Rect::new(x,y,256,256),self.rot,None,false,false).
    expect("oh no the stamp broke");
    cnv.present();

    self.rot += 1.0;
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
  let mut scr = sys.new_screen("borto",600,600)?;
  let tc = scr.texture_creator();
  let mut g = Gui::new(&tc);

  while sys.handle_events(eventer) {
    scr.frame(&mut g,sys.now());
  }
  Ok(())
}
