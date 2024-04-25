use super::ButtonState;

//the sdl enum has a lot more variations
//that I'm not gonna support just yet
#[derive(Copy,Clone)]
pub enum MouseButton {
  LEFT,
  RIGHT,
  MIDDLE
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
			left_button: ButtonState::new(),
			right_button: ButtonState::new(),
			middle_button: ButtonState::new(),
			pos:(0,0),
			wheel_amount: 0
		}
	}

  pub fn get_btn(&self, btn: MouseButton) -> &ButtonState {
    match btn {
      MouseButton::LEFT => &self.left_button,
      MouseButton::RIGHT => &self.right_button,
      MouseButton::MIDDLE => &self.middle_button
    }
  }
}

