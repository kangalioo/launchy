#![allow(unused_imports)]

mod util;

pub mod launchpad_s;
pub use launchpad_s as s;

pub mod launchpad_mk2;
pub use launchpad_mk2 as mk2;


/// Identifier used for e.g. the midi port names etc.
const APPLICATION_NAME: &'static str = "LaunchpadRs";


#[derive(Debug, Copy, Clone, Hash, Eq, PartialEq)]
pub enum Button {
	ControlButton { number: u8 },
	GridButton { x: u8, y: u8 },
}

impl Button {
	pub const UP: Self = Button::ControlButton { number: 0 };
	pub const DOWN: Self = Button::ControlButton { number: 1 };
	pub const LEFT: Self = Button::ControlButton { number: 2 };
	pub const RIGHT: Self = Button::ControlButton { number: 3 };
	pub const SESSION: Self = Button::ControlButton { number: 4 };
	pub const USER_1: Self = Button::ControlButton { number: 5 };
	pub const USER_2: Self = Button::ControlButton { number: 6 };
	pub const MIXER: Self = Button::ControlButton { number: 7 };
	pub const VOLUME: Self = Button::GridButton { x: 8, y: 0 };
	pub const PAN: Self = Button::GridButton { x: 8, y: 1 };
	pub const SEND_A: Self = Button::GridButton { x: 8, y: 2 };
	pub const SEND_B: Self = Button::GridButton { x: 8, y: 3 };
	pub const STOP: Self = Button::GridButton { x: 8, y: 4 };
	pub const MUTE: Self = Button::GridButton { x: 8, y: 5 };
	pub const SOLO: Self = Button::GridButton { x: 8, y: 6 };
	pub const RECORD_ARM: Self = Button::GridButton { x: 8, y: 7 };

	/// Returns x coordinate assuming coordinate origin in the leftmost control button
	pub fn abs_x(&self) -> u8 {
		match self {
			Self::ControlButton { number } => return *number,
			Self::GridButton { x, .. } => return *x,
		}
	}

	/// Returns y coordinate assuming coordinate origin in the leftmost control button
	pub fn abs_y(&self) -> u8 {
		match self {
			Self::ControlButton { .. } => return 0,
			Self::GridButton { y, .. } => y + 1,
		}
	}
}