use std::error::Error;
use sdl2::event::Event;
use sdl2::render::Texture;
use sdl2::image::LoadTexture;
use sdl2::keyboard::Keycode;

use box_ui::UISystem;
use box_ui::UIError;
use box_ui::DrawContext; //using this for the trait impl
use box_ui::IOContext; //once again the traits!
use box_ui::FrameDrawer;
use box_ui::BoxFont;

#[path="./assets/pdos_12.rs"]
mod pdos_12;

struct Gui<'a,T:BoxFont> {
  fnt:T,
  txt:Texture<'a>,
  line:String,
  x:i32,
  y:i32,
}

impl<'a,T:BoxFont> Gui<'a,T> {
  pub fn new(fnt:T,mut txt:Texture<'a>,words:&str) -> Self {
    txt.set_alpha_mod(0xFF);
    Self {
      fnt:fnt,
      txt:txt,
      line:String::from(words),
      x:10,
      y:10
    }
  }
}

impl<'a,T:BoxFont,D:DrawContext> FrameDrawer<D,()> for Gui<'a,T> {
  fn draw_frame(&mut self,cnv: &mut D,io:&IOContext) -> Result<(),UIError> {
    cnv.set_rgba(0x00,0x00,0x00,0x00);
    cnv.clear();
    

    self.fnt.write(cnv,&self.txt,self.x,self.y,self.line.as_str())?;

    if io.key_down(Keycode::E) {
      cnv.set_rgba(0xFF,0x00,0xFF,0xFF);
      cnv.blend_mul();
      let (w,h) = self.fnt.size(self.line.as_str());
      cnv.fill_rectangle(self.x, self.y+(h as i32)/4,w,h/4)?;
      cnv.blend_off();
    }

    if io.key_press(Keycode::Up) {
      self.y -= 1;
    }

    if io.key_press(Keycode::Down) {
      self.y += 1;
    }

    if io.key_press(Keycode::Left) {
      self.x -= 1;
    }

    if io.key_press(Keycode::Right) {
      self.x += 1;
    }

    cnv.present();
    Ok(())
  }
}

fn main() -> Result<(),Box<dyn Error>> {
  let mut sys = UISystem::new()?;
  let mut scr = sys.new_screen("pangea",250,250)?;
  let tc = scr.texture_creator();

  //this is relative to where you are running things from
  //there has to be some stuff in the path library for this case
  //in the mean time just run this from the project root
  let txt = tc.load_texture("./examples/assets/perfect_dos_vga_437_regular_12.png")?;
  let pdos = pdos_12::info();

  let mut g = Gui::new(pdos,txt,"WOW LOOK ITS TEXT!\nso much text");

  while sys.handle_events(|ev|{
    match ev {
      Event::Quit{..} => false,
      e=>scr.process_event(&e)
    }
  }) {
    scr.frame(&mut g,sys.now()).expect("oh no frame drawing failed");
  }

  Ok(())
}
