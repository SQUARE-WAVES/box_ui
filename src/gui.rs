use super::DrawContext;
use super::IOContext;
use super::BoxFont;

pub mod buttons;

pub trait Widget
  fn bounds(&self) -> (i32,i32,u32,u32);
  fn draw<D:DrawContext>(&mut self, dc: &mut D, io: &IOContext);
}

pub trait Resizable
  fn set_bounds(&self,bounds: (i32,i32,u32,u32));
}

pub fn render_widget<W:Widget,D:DrawContext>(widget: &mut W, dc:&mut D, io: &IOContext) {
  let restore_port = dc.get_portal();
  dc.set_portal(w.bounds());
  let io_cut = io.cut(w.bounds());

  w.draw(dc,io);

  dc.set_portal(restore_port);
}
