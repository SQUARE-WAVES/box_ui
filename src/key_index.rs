use sdl2::keyboard::Keycode;

pub const KEYCODE_RANGE : (usize,usize) = (0,234);

pub fn index(kc: &Keycode) -> usize {
	match kc {
		Keycode::Backspace => 0,
		Keycode::Tab => 1,
		Keycode::Return => 2,
		Keycode::Escape => 3,
		Keycode::Space => 4,
		Keycode::Exclaim => 5,
		Keycode::Quotedbl => 6,
		Keycode::Hash => 7,
		Keycode::Dollar => 8,
		Keycode::Percent => 9,
		Keycode::Ampersand => 10,
		Keycode::Quote => 11,
		Keycode::LeftParen => 12,
		Keycode::RightParen => 13,
		Keycode::Asterisk => 14,
		Keycode::Plus => 15,
		Keycode::Comma => 16,
		Keycode::Minus => 17,
		Keycode::Period => 18,
		Keycode::Slash => 19,
		Keycode::Num0 => 20,
		Keycode::Num1 => 21,
		Keycode::Num2 => 22,
		Keycode::Num3 => 23,
		Keycode::Num4 => 24,
		Keycode::Num5 => 25,
		Keycode::Num6 => 26,
		Keycode::Num7 => 27,
		Keycode::Num8 => 28,
		Keycode::Num9 => 29,
		Keycode::Colon => 30,
		Keycode::Semicolon => 31,
		Keycode::Less => 32,
		Keycode::Equals => 33,
		Keycode::Greater => 34,
		Keycode::Question => 35,
		Keycode::At => 36,
		Keycode::LeftBracket => 37,
		Keycode::Backslash => 38,
		Keycode::RightBracket => 39,
		Keycode::Caret => 40,
		Keycode::Underscore => 41,
		Keycode::Backquote => 42,
		Keycode::A => 43,
		Keycode::B => 44,
		Keycode::C => 45,
		Keycode::D => 46,
		Keycode::E => 47,
		Keycode::F => 48,
		Keycode::G => 49,
		Keycode::H => 50,
		Keycode::I => 51,
		Keycode::J => 52,
		Keycode::K => 53,
		Keycode::L => 54,
		Keycode::M => 55,
		Keycode::N => 56,
		Keycode::O => 57,
		Keycode::P => 58,
		Keycode::Q => 59,
		Keycode::R => 60,
		Keycode::S => 61,
		Keycode::T => 62,
		Keycode::U => 63,
		Keycode::V => 64,
		Keycode::W => 65,
		Keycode::X => 66,
		Keycode::Y => 67,
		Keycode::Z => 68,
		Keycode::Delete => 69,
		Keycode::CapsLock => 70,
		Keycode::F1 => 71,
		Keycode::F2 => 72,
		Keycode::F3 => 73,
		Keycode::F4 => 74,
		Keycode::F5 => 75,
		Keycode::F6 => 76,
		Keycode::F7 => 77,
		Keycode::F8 => 78,
		Keycode::F9 => 79,
		Keycode::F10 => 80,
		Keycode::F11 => 81,
		Keycode::F12 => 82,
		Keycode::PrintScreen => 83,
		Keycode::ScrollLock => 84,
		Keycode::Pause => 85,
		Keycode::Insert => 86,
		Keycode::Home => 87,
		Keycode::PageUp => 88,
		Keycode::End => 89,
		Keycode::PageDown => 90,
		Keycode::Right => 91,
		Keycode::Left => 92,
		Keycode::Down => 93,
		Keycode::Up => 94,
		Keycode::NumLockClear => 95,
		Keycode::KpDivide => 96,
		Keycode::KpMultiply => 97,
		Keycode::KpMinus => 98,
		Keycode::KpPlus => 99,
		Keycode::KpEnter => 100,
		Keycode::Kp1 => 101,
		Keycode::Kp2 => 102,
		Keycode::Kp3 => 103,
		Keycode::Kp4 => 104,
		Keycode::Kp5 => 105,
		Keycode::Kp6 => 106,
		Keycode::Kp7 => 107,
		Keycode::Kp8 => 108,
		Keycode::Kp9 => 109,
		Keycode::Kp0 => 110,
		Keycode::KpPeriod => 111,
		Keycode::Application => 112,
		Keycode::Power => 113,
		Keycode::KpEquals => 114,
		Keycode::F13 => 115,
		Keycode::F14 => 116,
		Keycode::F15 => 117,
		Keycode::F16 => 118,
		Keycode::F17 => 119,
		Keycode::F18 => 120,
		Keycode::F19 => 121,
		Keycode::F20 => 122,
		Keycode::F21 => 123,
		Keycode::F22 => 124,
		Keycode::F23 => 125,
		Keycode::F24 => 126,
		Keycode::Execute => 127,
		Keycode::Help => 128,
		Keycode::Menu => 129,
		Keycode::Select => 130,
		Keycode::Stop => 131,
		Keycode::Again => 132,
		Keycode::Undo => 133,
		Keycode::Cut => 134,
		Keycode::Copy => 135,
		Keycode::Paste => 136,
		Keycode::Find => 137,
		Keycode::Mute => 138,
		Keycode::VolumeUp => 139,
		Keycode::VolumeDown => 140,
		Keycode::KpComma => 141,
		Keycode::KpEqualsAS400 => 142,
		Keycode::AltErase => 143,
		Keycode::Sysreq => 144,
		Keycode::Cancel => 145,
		Keycode::Clear => 146,
		Keycode::Prior => 147,
		Keycode::Return2 => 148,
		Keycode::Separator => 149,
		Keycode::Out => 150,
		Keycode::Oper => 151,
		Keycode::ClearAgain => 152,
		Keycode::CrSel => 153,
		Keycode::ExSel => 154,
		Keycode::Kp00 => 155,
		Keycode::Kp000 => 156,
		Keycode::ThousandsSeparator => 157,
		Keycode::DecimalSeparator => 158,
		Keycode::CurrencyUnit => 159,
		Keycode::CurrencySubUnit => 160,
		Keycode::KpLeftParen => 161,
		Keycode::KpRightParen => 162,
		Keycode::KpLeftBrace => 163,
		Keycode::KpRightBrace => 164,
		Keycode::KpTab => 165,
		Keycode::KpBackspace => 166,
		Keycode::KpA => 167,
		Keycode::KpB => 168,
		Keycode::KpC => 169,
		Keycode::KpD => 170,
		Keycode::KpE => 171,
		Keycode::KpF => 172,
		Keycode::KpXor => 173,
		Keycode::KpPower => 174,
		Keycode::KpPercent => 175,
		Keycode::KpLess => 176,
		Keycode::KpGreater => 177,
		Keycode::KpAmpersand => 178,
		Keycode::KpDblAmpersand => 179,
		Keycode::KpVerticalBar => 180,
		Keycode::KpDblVerticalBar => 181,
		Keycode::KpColon => 182,
		Keycode::KpHash => 183,
		Keycode::KpSpace => 184,
		Keycode::KpAt => 185,
		Keycode::KpExclam => 186,
		Keycode::KpMemStore => 187,
		Keycode::KpMemRecall => 188,
		Keycode::KpMemClear => 189,
		Keycode::KpMemAdd => 190,
		Keycode::KpMemSubtract => 191,
		Keycode::KpMemMultiply => 192,
		Keycode::KpMemDivide => 193,
		Keycode::KpPlusMinus => 194,
		Keycode::KpClear => 195,
		Keycode::KpClearEntry => 196,
		Keycode::KpBinary => 197,
		Keycode::KpOctal => 198,
		Keycode::KpDecimal => 199,
		Keycode::KpHexadecimal => 200,
		Keycode::LCtrl => 201,
		Keycode::LShift => 202,
		Keycode::LAlt => 203,
		Keycode::LGui => 204,
		Keycode::RCtrl => 205,
		Keycode::RShift => 206,
		Keycode::RAlt => 207,
		Keycode::RGui => 208,
		Keycode::Mode => 209,
		Keycode::AudioNext => 210,
		Keycode::AudioPrev => 211,
		Keycode::AudioStop => 212,
		Keycode::AudioPlay => 213,
		Keycode::AudioMute => 214,
		Keycode::MediaSelect => 215,
		Keycode::Www => 216,
		Keycode::Mail => 217,
		Keycode::Calculator => 218,
		Keycode::Computer => 219,
		Keycode::AcSearch => 220,
		Keycode::AcHome => 221,
		Keycode::AcBack => 222,
		Keycode::AcForward => 223,
		Keycode::AcStop => 224,
		Keycode::AcRefresh => 225,
		Keycode::AcBookmarks => 226,
		Keycode::BrightnessDown => 227,
		Keycode::BrightnessUp => 228,
		Keycode::DisplaySwitch => 229,
		Keycode::KbdIllumToggle => 230,
		Keycode::KbdIllumDown => 231,
		Keycode::KbdIllumUp => 232,
		Keycode::Eject => 233,
		Keycode::Sleep => 234
	}
}
