use sdl2::rect::Rect;
use sdl2::keyboard::Keycode;

use super::io_state::IOState;
use super::io_state::ButtonPosition::{ DOWN, UP };
use super::io_state::ButtonState;
use super::io_state::MouseButton;

pub struct IOContext<'a> {
  state: &'a IOState,
  bounds: Rect //using the sdl type to avoid .intos which may cost some
}

//these are done in separate impl blocks just to keep it easy to read

//basic creation and utilities
impl<'a> IOContext<'a> {
  pub fn new(st:&'a IOState, x:i32,y:i32,w:u32,h:u32) -> Self {
    Self{
      state:st,
      bounds:Rect::new(x,y,w,h)
    }
  }

  pub fn cut(&self,(x,y,w,h):(i32,i32,u32,u32)) -> Self {
    let (cx,cy) = self.relative_pos((x,y));
    Self{
      state:self.state,
      bounds:Rect::new(cx,cy,w,h)
    }
  }

  pub fn bounds(&self) -> (i32,i32,u32,u32) {
    self.bounds.into()
  }
  
  fn current_mouse(&self,btn:MouseButton) -> &ButtonState {
    self.state.get_current_mouse_state(btn)
  }

  fn prev_mouse(&self,btn:MouseButton) -> &ButtonState {
    self.state.get_prev_mouse_state(btn)
  }

  fn current_key(&self,key_code : Keycode) -> &ButtonState {
		&self.state.key_map.get_state(&key_code).new
  }

  fn prev_key(&self,key_code : Keycode) -> &ButtonState {
		&self.state.key_map.get_state(&key_code).old
  }

  fn relative_pos(&self,(x,y):(i32,i32)) -> (i32,i32) {
    (x-self.bounds.x(),y-self.bounds.y())
  }
}

//mouse clicks and wheels
impl<'a> IOContext<'a> {
  pub fn down_pos(&self, btn:MouseButton) -> (i32,i32) {
    self.relative_pos(self.current_mouse(btn).down_pos)
  }

  pub fn prev_down_pos(&self, btn:MouseButton) -> (i32,i32) {
    self.relative_pos(self.prev_mouse(btn).down_pos)
  }

  pub fn up_pos(&self, btn:MouseButton) -> (i32,i32) {
    self.relative_pos(self.current_mouse(btn).up_pos)
  }

  pub fn prev_up_pos(&self, btn:MouseButton) -> (i32,i32) {
    self.relative_pos(self.prev_mouse(btn).up_pos)
  }

	pub fn mouse_down(&self,btn:MouseButton) -> bool {
    let btn_state =self.current_mouse(btn);
    btn_state.button == DOWN && self.bounds.contains_point(btn_state.down_pos)
	}

  pub fn left_down(&self) -> bool {
    self.mouse_down(MouseButton::LEFT)
  }

	pub fn right_down(&self) -> bool {
    self.mouse_down(MouseButton::RIGHT)
  }

	pub fn middle_down(&self) -> bool {
    self.mouse_down(MouseButton::MIDDLE)
  }

	pub fn end_click(&self,btn: MouseButton) -> bool  {
    let state = self.current_mouse(btn);
    let pre_state = self.prev_mouse(btn);
		let click = state.button == UP && pre_state.button == DOWN;
    let up_in = self.bounds.contains_point(state.up_pos);
    let down_in = self.bounds.contains_point(pre_state.down_pos);

    click && up_in && down_in
	}
	
  pub fn start_click(&self, btn: MouseButton) -> bool  {
    let state = self.current_mouse(btn);
    let pre_state = self.prev_mouse(btn);

		let click = state.button == DOWN && pre_state.button == UP;
    let down_in = self.bounds.contains_point(state.down_pos);

    click && down_in
	}

  pub fn end_left_click(&self) -> bool {
    self.end_click(MouseButton::LEFT)
  }
  
  pub fn start_left_click(&self) -> bool {
    self.start_click(MouseButton::LEFT)
  }
	
  pub fn end_right_click(&self) -> bool {
    self.end_click(MouseButton::RIGHT)
  }

  pub fn start_right_click(&self) -> bool {
    self.start_click(MouseButton::RIGHT)
  }

	pub fn end_middle_click(&self) -> bool {
    self.end_click(MouseButton::MIDDLE)
  }

	pub fn start_middle_click(&self) -> bool{
    self.start_click(MouseButton::MIDDLE)
  }

	pub fn wheelage(&self) -> i32 {
		self.state.current_mouse.wheel_amount
	}
}

//mouse movement and position
impl<'a> IOContext<'a> {
	pub fn mouse_delta(&self) -> (i32,i32) {
		let (curr_x,curr_y) = self.state.current_mouse.pos;
		let (prev_x,prev_y) = self.state.prev_mouse.pos;

		(curr_x - prev_x,curr_y - prev_y)
	}

  pub fn mouse_x_delta(&self) -> i32 {
    let (x,_) = self.mouse_delta();
    x
  }

  pub fn mouse_y_delta(&self) -> i32 {
    let(_,y) = self.mouse_delta();
    y
  }

	pub fn mouse_pos(&self) -> (i32,i32) {
	  self.relative_pos(self.state.current_mouse.pos)
	}
}

//keyboard! right now it's all global
impl<'a> IOContext<'a> {
	pub fn key_down(&self, key_code : Keycode) -> bool {
		self.current_key(key_code).button == DOWN
	}

	pub fn key_up(&self, key_code : Keycode) -> bool {
		self.current_key(key_code).button == UP
	}

	pub fn key_press(&self, key_code : Keycode) -> bool {
		let new = self.current_key(key_code).button == DOWN;
		let old = self.prev_key(key_code).button == UP;

    old && new
	}

	pub fn key_release(&self, key_code : Keycode) -> bool {
    let new = self.current_key(key_code).button == UP;
		let old = self.prev_key(key_code).button == DOWN;

    new && old
  }

  //TIME IS IN HERE TOO! WOW
  pub fn now(&self) -> u64 {
    self.state.current_time
  }

  pub fn delta_t(&self) -> u64 {
    self.state.current_time - self.state.prev_time
  }
}


/*
impl<'a,T:IOContext> IOContext for IOPortal<'a,T> {
  fn bounds(&self) -> Rect {
    self.bounds
  }

  fn portal(&self,x:i32,y:i32,w:u32,h:u32) -> IOPortal<Self> {
    IOPortal{io:self,bounds:Rect::new(x,y,w,h)} 
  }

  fn down_pos(&self, btn:MouseButton) -> (i32,i32) {
    let (x,y) = self.io.down_pos(btn);
    (x-self.bounds.x(),y-self.bounds.y())
  }

  fn up_pos(&self,btn:MouseButton) -> (i32,i32) {
    let (x,y) = self.io.up_pos(btn);
    (x-self.bounds.x(),y-self.bounds.y())
  }

  fn prev_down_pos(&self, btn:MouseButton) -> (i32,i32) {
    let (x,y) = self.io.prev_down_pos(btn);
    (x-self.bounds.x(),y-self.bounds.y())
  }

  fn prev_up_pos(&self,btn:MouseButton) -> (i32,i32) {
    let (x,y) = self.io.prev_up_pos(btn);
    (x-self.bounds.x(),y-self.bounds.y())
  }

	fn mouse_down(&self,btn:MouseButton) -> bool {
    self.io.mouse_down(btn) && self.bounds().contains_point(self.io.down_pos(btn))
	}

	fn end_click(&self,btn: MouseButton) -> bool  {
    let click = self.io.end_click(btn);
    let down_in = self.bounds.contains_point(self.io.prev_down_pos(btn));
    let up_in = self.bounds.contains_point(self.io.up_pos(btn));

    click && down_in && up_in
	}
	
  fn start_click(&self, btn: MouseButton) -> bool  {
    let click = self.io.start_click(btn);
    let down_in = self.bounds.contains_point(self.io.prev_down_pos(btn));
    
    click && down_in
	}

	//for deltas relative and absolute are the same
	fn mouse_delta(&self) -> (i32,i32) {
    self.io.mouse_delta()
	}

  //this is relative to the portal!
	fn mouse_pos(&self) -> (i32,i32) {
	  let (gx,gy) = self.io.mouse_pos();
    (gx-self.bounds.x(),gy-self.bounds.y())
	}

	fn wheelage(&self) -> i32 {
		self.io.wheelage()
	}

	fn key_down(&self, key_code : Keycode) -> bool {
		self.io.key_down(key_code)
	}

	fn key_up(&self, key_code : Keycode) -> bool {
    self.io.key_up(key_code)
	}

	fn key_press(&self, key_code : Keycode) -> bool {
    self.io.key_press(key_code)
	}

	fn key_release(&self, key_code : Keycode) -> bool {
    self.io.key_release(key_code)
	}

  fn now(&self) -> u64 {
    self.io.now()
  }

  fn delta_t(&self) -> u64 {
    self.io.delta_t()
  }
}*/
