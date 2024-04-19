use sdl2::event::Event;
use sdl2::mouse::MouseButton;

use super::key_map::KeyMap;

#[derive(Copy,Clone,PartialEq,Eq)]
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

#[derive(Copy,Clone)]
pub struct MouseState {
	pub left_button: ButtonState,
	pub right_button: ButtonState,
	pub middle_button: ButtonState,
	pub pos:(i32,i32),
	pub wheel_amount: i32
}

impl MouseState {
	pub fn new() -> Self {
		MouseState {
			left_button:ButtonState::new(),
			right_button: ButtonState::new(),
			middle_button: ButtonState::new(),
			pos:(0,0),
			wheel_amount: 0
		}
	}
}

pub struct IOState {
	pub current_mouse:MouseState,
	pub prev_mouse:MouseState,
	pub key_map:KeyMap
}

impl IOState {
	pub fn new() -> Self {
		IOState {
			current_mouse:MouseState::new(),
			prev_mouse:MouseState::new(),
			key_map:KeyMap::new()
		}
	}

	pub fn flip(&mut self) {
		self.prev_mouse = self.current_mouse;
	}

	pub fn process_event(&mut self, ev: &Event) -> bool {
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
			}
			_=>()
		};

    true
	}
}
