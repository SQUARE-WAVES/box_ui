use super::DrawContext;
use super::IOContext;
use super::UIError;

pub trait Widget<R,T> {
  fn size(&self) -> (u32,u32);

  fn draw<D:DrawContext>(&mut self, dc: &mut D, io: &IOContext,r:&mut R) -> Result<T,UIError>;
}

pub fn render_widget<R,T,W,D>(x:i32,y:i32,w: &mut W, dc:&mut D, io: &IOContext,r:&mut R) -> Result<T,UIError>
where
  W:Widget<R,T>,
  D:DrawContext
{
  let (width,height) = w.size();
  let bounds = (x,y,width,height);

  let restore_port = dc.get_portal();
  dc.set_portal(bounds);
  let io_cut = io.cut(bounds);

  let out = w.draw(dc,&io_cut,r);

  dc.set_portal(restore_port);

  out
}
