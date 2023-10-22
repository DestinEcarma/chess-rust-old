use std::{
	fmt::{Display, Formatter, Result},
	ops::Not,
};

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum Color {
	White,
	Black,
}

impl From<char> for Color {
	fn from(value: char) -> Self {
		if value == value.to_ascii_uppercase() {
			Color::White
		} else {
			Color::Black
		}
	}
}

impl Not for Color {
	type Output = Color;

	fn not(self) -> Self {
		if self == Color::White {
			Color::Black
		} else {
			Color::White
		}
	}
}

impl Display for Color {
	fn fmt(&self, f: &mut Formatter<'_>) -> Result {
		write!(
			f,
			"{}",
			if self == &Color::White {
				"White"
			} else {
				"Black"
			}
		)
	}
}

impl Color {
	pub fn to_index(&self) -> usize {
		*self as usize
	}
}
