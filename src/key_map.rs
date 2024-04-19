use sdl2::keyboard::Keycode;
use std::vec::Vec;
use super::io_state::ButtonState;
use super::io_state::ButtonPosition;
use super::key_index;

#[derive(Copy,Clone)]
pub struct KeyState {
	pub old: ButtonState,
	pub new: ButtonState
}

impl KeyState  {
	
	pub fn new() -> Self {
		KeyState {
			old: ButtonState::new(),
			new: ButtonState::new()
		}
	}
}

pub struct KeyMap {
	keys:Vec<KeyState>
}

impl KeyMap {
	pub fn new() -> Self {
		KeyMap {
			keys: vec![KeyState::new();key_index::KEYCODE_RANGE.1]
		}
	}

	pub fn get_state(&self,kc: &Keycode) -> &KeyState {
		let idx = key_index::index(kc);
		self.keys.get(idx).unwrap() //panics if we are out of range
	}

	pub fn flip(&mut self) {
		for state in self.keys.iter_mut() {
			state.old = state.new;
		}
	}

	pub fn set_state_down(&mut self, kc: &Keycode, pos:(i32,i32)) {
		let idx = key_index::index(kc);
		let state = self.keys.get_mut(idx).unwrap();
		
		state.new.button = ButtonPosition::DOWN;
		state.new.down_pos = pos;
	}

	pub fn set_state_up(&mut self, kc: &Keycode, pos:(i32,i32)) {
		let idx = key_index::index(kc);
		let state = self.keys.get_mut(idx).unwrap();
		
		state.new.button = ButtonPosition::UP;
		state.new.up_pos = pos;
	}
}
