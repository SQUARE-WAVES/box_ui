use std::cmp::min;
use std::cmp::max;

use sdl2::rect::Rect;
use sdl2::keyboard::Keycode;

use super::io_state::IOState;
use super::io_state::ButtonPosition::{ DOWN, UP };

fn clamp<T :Ord>(v: T, minv: T, maxv: T) -> T {
  max(minv,min(v,maxv))
}

pub struct IOContext<'a> {
	state: &'a IOState,
	bounds:Rect
}

impl<'a> IOContext<'a> {
	pub fn new(st: &'a IOState, x:i32,y:i32,w:u32,h:u32) -> Self {
		Self {
			state:st,
			bounds:(x,y,w,h).into()
		}
	}

  //this returns another IO context that has subset bounds
  //it's for doing composites 
  pub fn cut(parent: &'a IOContext,x:i32,y:i32,w:u32,h:u32) -> Self {
    let (rx,ry,rw,rh) = parent.bounds.into();
    //gotta do some clamping here
    let cut_x = clamp(x+rx,0,rx+rw as i32);
    let cut_y = clamp(y+ry,0,ry+rh as i32);
    let cut_w = clamp(w,0,w-x as u32);
    let cut_h = clamp(h,0,h-y as u32);

    Self {
      state:parent.state,
      bounds:(cut_x,cut_y,cut_w,cut_h).into()
    }
  }

  fn contains_point<T:Into<(i32,i32)>>(&self,p: T) -> bool {
    self.bounds.contains_point(p)
  }

	//is the left mouse button down inside our widget
	pub fn left_down(&self) -> bool {
		let state = self.state.current_mouse.left_button;
		(state.button == DOWN) && self.contains_point(state.down_pos)
	}

	pub fn right_down(&self) -> bool {
		let state = self.state.current_mouse.right_button;
		(state.button == DOWN) && self.contains_point(state.down_pos)
	}

	pub fn middle_down(&self) -> bool {
		let state = self.state.current_mouse.middle_button;
		(state.button == DOWN) && self.contains_point(state.down_pos)
	}

	pub fn end_click(&self) -> bool  {
		let state = self.state.current_mouse.left_button;
		let pre_state = self.state.prev_mouse.left_button;
		let button_up = state.button == UP && pre_state.button == DOWN;

		let down_in_bounds = self.contains_point(pre_state.down_pos);
		let up_in_bounds = self.contains_point(state.up_pos);

		button_up && down_in_bounds && up_in_bounds
	}

	pub fn end_right_click(&self) -> bool {
		let state = self.state.current_mouse.right_button;
		let pre_state = self.state.prev_mouse.right_button;
		let button_up = state.button == UP && pre_state.button == DOWN;

		let down_in_bounds = self.contains_point(pre_state.down_pos);
		let up_in_bounds = self.contains_point(state.up_pos);

		button_up && down_in_bounds && up_in_bounds
	}

	pub fn end_middle_click(&self) -> bool {
		let state = self.state.current_mouse.middle_button;
		let pre_state = self.state.prev_mouse.middle_button;
		let button_up = state.button == UP && pre_state.button == DOWN;

		let down_in_bounds = self.contains_point(pre_state.down_pos);
		let up_in_bounds = self.contains_point(state.up_pos);

		button_up && down_in_bounds && up_in_bounds
	}

	pub fn left_click_out(&self) -> bool {
		let state = self.state.current_mouse.left_button;
		state.button == DOWN && !self.contains_point(state.down_pos)
	}

	pub fn right_click_out(&self) -> bool {
		let state = self.state.current_mouse.right_button;
		state.button == DOWN && !self.contains_point(state.down_pos)
	}

	pub fn middle_click_out(&self) -> bool {
		let state = self.state.current_mouse.middle_button;

		state.button == DOWN && !self.contains_point(state.down_pos)
	}

	//for deltas relative and absolute are the same
	pub fn mouse_y_delta(&self) -> i32 {
		let (_,curr_y) = self.state.current_mouse.pos;
		let (_,prev_y) = self.state.prev_mouse.pos;

		curr_y - prev_y
	}

	pub fn mouse_x_delta(&self) -> i32 {
		let (curr_x,_) = self.state.current_mouse.pos;
		let (prev_x,_) = self.state.prev_mouse.pos;

		curr_x - prev_x
	}

	//these get the relative position to the widget
	pub fn mouse_y_pos(&self) -> i32 {
		let (_,y) = self.state.current_mouse.pos;
		let (_,off_y,_,_) = self.bounds.into();
		
		y - off_y
	}

	pub fn mouse_x_pos(&self) -> i32 {
		let (x,_) = self.state.current_mouse.pos;
		let (off_x,_,_,_) = self.bounds.into();
		
		x - off_x
	}

	pub fn mouse_pos(&self) -> (i32,i32) {
		let (x,y) = self.state.current_mouse.pos;
		let (off_x,off_y,_,_) = self.bounds.into();
		
		(x-off_x,y-off_y)
	}

	pub fn wheelage(&self) -> i32 {
		return self.state.current_mouse.wheel_amount;
	}

	pub fn key_down(&self, key_code : Keycode) -> bool {
		return self.state.key_map.get_state(&key_code).new.button == DOWN;
	}

	pub fn key_up(&self, key_code : Keycode) -> bool {
		return self.state.key_map.get_state(&key_code).new.button == UP;	
	}
}
