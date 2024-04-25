use std::collections::HashMap;
use std::iter::FromIterator;

use box_ui::BoxFont;
use box_ui::LetterInfo;

pub struct Pdos12 {
  charmap:HashMap<char,LetterInfo>,
  default_char: LetterInfo
}

fn letter_info(o:i32,o1:i32,s:i32,s1:i32,s2:u32,s3:u32) -> LetterInfo {
  (o,o1,s,s1,s2,s3).into()
}

pub fn info() -> Pdos12  {
	let map = HashMap::from_iter(vec![
		(' ', letter_info(0,12-0,1,13,0,0)),
		('!', letter_info(2,12-10,2,3,4,10)),
		('"', letter_info(1,12-11,7,2,6,4)),
		('#', letter_info(0,12-9,14,4,7,9)),
		('$', letter_info(0,12-12,22,1,7,14)),
		('%', letter_info(0,12-8,30,5,7,8)),
		('&', letter_info(0,12-10,38,3,7,10)),
		('\'', letter_info(1,12-11,46,2,3,4)),
		('(', letter_info(2,12-10,50,3,4,10)),
		(')', letter_info(2,12-10,55,3,4,10)),
		('*', letter_info(0,12-7,60,6,8,5)),
		('+', letter_info(1,12-7,69,6,6,5)),
		(',', letter_info(2,12-3,76,10,3,4)),
		('-', letter_info(0,12-5,80,8,7,1)),
		('.', letter_info(3,12-2,88,11,2,2)),
		('/', letter_info(0,12-8,91,5,7,8)),
		('0', letter_info(0,12-10,99,3,7,10)),
		('1', letter_info(1,12-10,107,3,6,10)),
		('2', letter_info(0,12-10,114,3,7,10)),
		('3', letter_info(0,12-10,1,16,7,10)),
		('4', letter_info(0,12-10,9,16,7,10)),
		('5', letter_info(0,12-10,17,16,7,10)),
		('6', letter_info(0,12-10,25,16,7,10)),
		('7', letter_info(0,12-10,33,16,7,10)),
		('8', letter_info(0,12-10,41,16,7,10)),
		('9', letter_info(0,12-10,49,16,7,10)),
		(':', letter_info(3,12-8,57,18,2,7)),
		(';', letter_info(2,12-8,60,18,3,8)),
		('<', letter_info(1,12-9,64,17,6,9)),
		('=', letter_info(1,12-7,71,19,6,4)),
		('>', letter_info(1,12-9,78,17,6,9)),
		('?', letter_info(0,12-10,85,16,7,10)),
		('@', letter_info(0,12-9,93,17,7,9)),
		('A', letter_info(0,12-10,101,16,7,10)),
		('B', letter_info(0,12-10,109,16,7,10)),
		('C', letter_info(0,12-10,117,16,7,10)),
		('D', letter_info(0,12-10,1,27,7,10)),
		('E', letter_info(0,12-10,9,27,7,10)),
		('F', letter_info(0,12-10,17,27,7,10)),
		('G', letter_info(0,12-10,25,27,7,10)),
		('H', letter_info(0,12-10,33,27,7,10)),
		('I', letter_info(2,12-10,41,27,4,10)),
		('J', letter_info(0,12-10,46,27,7,10)),
		('K', letter_info(0,12-10,54,27,7,10)),
		('L', letter_info(0,12-10,62,27,7,10)),
		('M', letter_info(0,12-10,70,27,7,10)),
		('N', letter_info(0,12-10,78,27,7,10)),
		('O', letter_info(0,12-10,86,27,7,10)),
		('P', letter_info(0,12-10,94,27,7,10)),
		('Q', letter_info(0,12-10,102,27,7,12)),
		('R', letter_info(0,12-10,110,27,7,10)),
		('S', letter_info(0,12-10,118,27,7,10)),
		('T', letter_info(1,12-10,1,42,6,10)),
		('U', letter_info(0,12-10,8,42,7,10)),
		('V', letter_info(0,12-10,16,42,7,10)),
		('W', letter_info(0,12-10,24,42,7,10)),
		('X', letter_info(0,12-10,32,42,7,10)),
		('Y', letter_info(1,12-10,40,42,6,10)),
		('Z', letter_info(0,12-10,47,42,7,10)),
		('[', letter_info(2,12-10,55,42,4,10)),
		('\\', letter_info(0,12-9,60,43,7,9)),
		(']', letter_info(2,12-10,68,42,4,10)),
		('^', letter_info(0,12-12,73,40,7,4)),
		('_', letter_info(0,12+1,81,53,8,1)),
		('`', letter_info(2,12-11,90,41,4,3)),
		('a', letter_info(0,12-7,95,45,7,7)),
		('b', letter_info(0,12-10,103,42,7,10)),
		('c', letter_info(0,12-7,111,45,7,7)),
		('d', letter_info(0,12-10,119,42,7,10)),
		('e', letter_info(0,12-7,1,58,7,7)),
		('f', letter_info(1,12-10,9,55,6,10)),
		('g', letter_info(0,12-7,16,58,7,10)),
		('h', letter_info(0,12-10,24,55,7,10)),
		('i', letter_info(2,12-10,32,55,4,10)),
		('j', letter_info(1,12-10,37,55,6,13)),
		('k', letter_info(0,12-10,44,55,7,10)),
		('l', letter_info(2,12-10,52,55,4,10)),
		('m', letter_info(0,12-7,57,58,7,7)),
		('n', letter_info(0,12-7,65,58,7,7)),
		('o', letter_info(0,12-7,73,58,7,7)),
		('p', letter_info(0,12-7,81,58,7,10)),
		('q', letter_info(0,12-7,89,58,7,10)),
		('r', letter_info(0,12-7,97,58,7,7)),
		('s', letter_info(0,12-7,105,58,7,7)),
		('t', letter_info(0,12-10,113,55,7,10)),
		('u', letter_info(0,12-7,1,73,7,7)),
		('v', letter_info(0,12-7,9,73,7,7)),
		('w', letter_info(0,12-7,17,73,7,7)),
		('x', letter_info(0,12-7,25,73,7,7)),
		('y', letter_info(0,12-7,33,73,7,10)),
		('z', letter_info(0,12-7,41,73,7,7)),
		('{', letter_info(1,12-10,49,70,6,10)),
		('|', letter_info(3,12-10,56,70,2,10)),
		('}', letter_info(1,12-10,59,70,6,10)),
		('~', letter_info(0,12-11,66,69,7,2))
	]);

  Pdos12 {
		charmap:map,
    default_char:letter_info(0,12-0,1,13,0,0)
  }
}

impl BoxFont for Pdos12 { 
  fn letter_info(&self,c:&char) -> &LetterInfo {
    let mapped = self.charmap.get(c);
    
    //kind of a hack here, because this cannot fail, if you don't see the char
    //pretend it's a ' ' (space)
    mapped.unwrap_or(&self.default_char)
  }

  fn box_width(&self) -> u32 {
    9
  }

  fn box_height(&self) -> u32 {
    16
  }
}
