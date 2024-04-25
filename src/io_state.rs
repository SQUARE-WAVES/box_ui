use sdl2::event::Event;
use sdl2::event::WindowEvent;

mod key_map;
mod key_index;
mod mouse_state;

use key_map::KeyMap;
pub use mouse_state::MouseState;
pub use mouse_state::MouseButton;

//this is shared by both the mouse and keyboard states
#[derive(Copy,Clone,PartialEq,Eq,Debug)]
pub enum ButtonPosition {
	UP,
	DOWN,	
}

#[derive(Copy,Clone)]
pub struct ButtonState {
	pub button: ButtonPosition,
	pub down_pos:(i32,i32),
	pub up_pos:(i32,i32)
}

impl ButtonState {
	pub fn new() -> Self {
		ButtonState {
			button: ButtonPosition::UP,
			down_pos:(0,0),
			up_pos:(0,0)
		}
	}
}

pub struct IOState {
	pub current_mouse:MouseState,
	pub prev_mouse:MouseState,
	pub key_map:KeyMap,
  pub size: (u32,u32),
  pub current_time: u64,
  pub prev_time: u64,
  pub time_scale: u64
}

impl IOState {
	pub fn new(w:u32,h:u32,time_scale:u64) -> Self {
		IOState {
			current_mouse:MouseState::new(),
			prev_mouse:MouseState::new(),
			key_map:KeyMap::new(),
      size:(w,h),
      current_time:0,
      prev_time:0,
      time_scale:time_scale
		}
	}

	pub fn flip(&mut self) {
		self.prev_mouse = self.current_mouse;
    self.key_map.flip();
    self.prev_time = self.current_time;
	}

  //not sure how this is gonna work for context but WE WILL SEE
  fn process_window_event(&mut self,ev:&WindowEvent) -> bool {
    match ev {
      WindowEvent::Resized(new_w,new_h) => {
        self.size=(*new_w as u32,*new_h as u32);
        true
      },

      _=> true
    }
  }

	pub fn process_event(&mut self, ev: &Event) -> bool {
    //we are dealing with sdl events here, so use their mouse button enum
    use sdl2::mouse::MouseButton;
		match ev {
			Event::MouseMotion {x, y, ..} => {
				self.current_mouse.pos = (*x,*y);
			},

			Event::MouseButtonDown {mouse_btn,x, y,..} => {
				let alter_button_state = |st: &mut ButtonState,x:i32,y:i32| {
					st.button = ButtonPosition::DOWN;
					st.down_pos = (x,y);
				};

				match mouse_btn {
					MouseButton::Left => alter_button_state(&mut(self.current_mouse.left_button),*x,*y),
					MouseButton::Middle => alter_button_state(&mut(self.current_mouse.middle_button),*x,*y),
					MouseButton::Right => alter_button_state(&mut(self.current_mouse.right_button),*x,*y),
					_ => ()
				};
			},

			Event::MouseButtonUp {mouse_btn,x, y,..} =>  {
				let alter_button_state = |st: &mut ButtonState,x:i32,y:i32| {
					st.button = ButtonPosition::UP;
					st.up_pos = (x,y);
				};

				match mouse_btn {
					MouseButton::Left => alter_button_state(&mut(self.current_mouse.left_button),*x,*y),
					MouseButton::Middle => alter_button_state(&mut(self.current_mouse.middle_button),*x,*y),
					MouseButton::Right => alter_button_state(&mut(self.current_mouse.right_button),*x,*y),
					_ => ()
				};
			},
			
			Event::MouseWheel {y,..} => {
				self.current_mouse.wheel_amount = *y;
			},

			Event::KeyDown { keycode, ..} => {
				if let Some(kc) = keycode {
					self.key_map.set_state_down(kc,self.current_mouse.pos)
				}
			},

			Event::KeyUp { keycode, ..} => {
				if let Some(kc) = keycode {
					self.key_map.set_state_up(kc,self.current_mouse.pos)
				}
			},

      Event::Window {win_event, ..} => {
        self.process_window_event(&win_event);
      }
			_=>()
		};

    true
	}

  pub fn get_current_mouse_state (&self, btn: mouse_state::MouseButton) -> &ButtonState {
    self.current_mouse.get_btn(btn)
  }
  
  pub fn get_prev_mouse_state (&self, btn: mouse_state::MouseButton) -> &ButtonState {
    self.prev_mouse.get_btn(btn)
  }
}




