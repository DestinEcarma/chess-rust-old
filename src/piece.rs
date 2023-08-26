use crate::color::Color;
use std::fmt::{Display, Formatter, Result};

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum Piece {
	King,
	Pawn,
	Knight,
	Bishop,
	Rook,
	Queen,
}

pub const ALL_PIECES: [Piece; 6] = [
	Piece::King,
	Piece::Pawn,
	Piece::Knight,
	Piece::Bishop,
	Piece::Rook,
	Piece::Queen,
];

pub const PROMOTION_PIECES: [Piece; 4] = [Piece::Queen, Piece::Rook, Piece::Bishop, Piece::Bishop];

impl From<char> for Piece {
	fn from(char: char) -> Self {
		match char.to_ascii_lowercase() {
			'k' => Piece::King,
			'p' => Piece::Pawn,
			'n' => Piece::Knight,
			'b' => Piece::Bishop,
			'r' => Piece::Rook,
			'q' => Piece::Queen,
			_ => panic!(),
		}
	}
}

impl Display for Piece {
	fn fmt(&self, f: &mut Formatter) -> Result {
		write!(
			f,
			"{}",
			match self {
				Piece::King => "k",
				Piece::Pawn => "p",
				Piece::Knight => "n",
				Piece::Bishop => "b",
				Piece::Rook => "r",
				Piece::Queen => "q",
			}
		)
	}
}

impl Piece {
	pub fn to_index(&self) -> usize {
		*self as usize
	}

	pub fn to_string(&self, color: Color) -> String {
		let piece = format!("{}", self);

		if color == Color::White {
			piece.to_uppercase()
		} else {
			piece
		}
	}

	pub fn to_full_name(&self) -> String {
		match self {
			Piece::King => "King".to_string(),
			Piece::Pawn => "Pawn".to_string(),
			Piece::Knight => "Knight".to_string(),
			Piece::Bishop => "Bishop".to_string(),
			Piece::Rook => "Rook".to_string(),
			Piece::Queen => "Queen".to_string(),
		}
	}
}
