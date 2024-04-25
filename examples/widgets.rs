use std::error::Error;
use sdl2::event::Event;
use sdl2::render::Texture;
use sdl2::image::LoadTexture;

use box_ui::UISystem;
use box_ui::UIError;
use box_ui::DrawContext; //using this for the trait impl
use box_ui::IOContext; //once again the traits!
use box_ui::FrameDrawer;
use box_ui::BoxFont;
use box_ui::Widget;
use box_ui::render_widget;

#[path="./assets/pdos_12.rs"]
mod pdos_12;

//=============================================================================
//This one is a little big, the point is to show off designing widgets
//the gui module here gives you tools to do that, but doesn't yet
//provide a big library of preset widgets, the idea is each program
//might want custom widgets, which is easier than writing super general
//purpose widgets, if it comes to wanting to do that a second library
//can be made for doing that

//Here is our widget, it's just a button
//when you click it, it prints it's words
struct Btn{
  w:u32,
  h:u32,
  pub words:&'static str,
  tx:i32,
  ty:i32
}

impl Btn {

  fn new<T:BoxFont>(w:u32,h:u32,words:&'static str,fnt:&T) -> Self {
   
    let (tw,th)= fnt.size(words);
    let txt_x = ((w - tw) as i32)/2;
    let txt_y = ((h - th) as i32)/2;
    
    Self{
      w:w,
      h:h,
      words:words,
      tx:txt_x,
      ty:txt_y
    }
  }
}

impl<T:BoxFont> Widget<(&mut Texture<'_>,&T),bool> for Btn {

  fn size(&self) -> (u32,u32) { 
    (self.w,self.h)
  }

  fn draw<D>(&mut self,dc:&mut D, io: &IOContext,(txt,fnt):&mut (&mut Texture,&T)) -> Result<bool,UIError>
  where 
    D: DrawContext
  {
    let (color,(fnt_r,fnt_g,fnt_b)) = match io.left_down() {
      true => ((0x55,0xFF,0xFF,0xFF),(0x00,0x00,0x00)),
      false => ((0x00,0x00,0x00,0xFF),(0x55,0xFF,0xFF))
    };
  
    //draw_the_interor
    dc.set_color(color);
    dc.fill_rectangle(0,0,self.w,self.h)?;

    //draw the words
    txt.set_color_mod(fnt_r,fnt_g,fnt_b);
    fnt.write(dc,txt,self.tx,self.ty,self.words)?;

    //finally draw the border
    dc.set_rgba(0xFF,0x55,0xFF,0xFF);
    dc.draw_rectangle(0,0,self.w,self.h)?;
    
    Ok(io.end_left_click())
  }
}

pub struct Gui<'a,T:BoxFont> {
  txt:Texture<'a>,
  fnt:T,
  btn:Btn,
  btn2:Btn,
  btn3:Btn,
}

impl<'a,T:BoxFont,D:DrawContext> FrameDrawer<D,bool> for Gui<'a,T> {
  fn draw_frame(&mut self,cnv: &mut D,io:&IOContext) -> Result<bool,UIError> {
    cnv.set_rgba(0x00,0x00,0x00,0x00);
    cnv.clear();

    let mut resources = (&mut self.txt,&self. fnt);

    if render_widget(10,10,&mut self.btn,cnv,io,&mut resources)? {
      println!("CLICK: {}",self.btn.words);
    }

    if render_widget(100,10,&mut self.btn2,cnv,io,&mut resources)? {
      return Ok(true);
    }

    if render_widget(10,45,&mut self.btn3,cnv,io,&mut resources)? {
      println!("BOING!!!!!");
    }

    cnv.present();
    Ok(false)
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

  let btn = Btn::new(80,30,"glorpus",&pdos);
  let btn2 = Btn::new(40,30,"[X]",&pdos);
  let btn3 = Btn::new(20,20,"!",&pdos);

  let mut g = Gui{txt:txt,fnt:pdos,btn:btn,btn2:btn2,btn3:btn3};

  while sys.handle_events(|ev|{
    match ev {
      Event::Quit{..} => false,
      e=>scr.process_event(&e)
    }
  }) {
    if scr.frame(&mut g,sys.now())? {
      break;
    }
  }
  

  Ok(())
}
