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
	Piece::King,   //* 0b0000 */
	Piece::Pawn,   //* 0b0001 */
	Piece::Knight, //* 0b0010 */
	Piece::Bishop, //* 0b0011 */
	Piece::Rook,   //* 0b0100 */
	Piece::Queen,  //* 0b0101 */
];

pub const PROMOTION_PIECES: [Piece; 4] = [Piece::Queen, Piece::Rook, Piece::Bishop, Piece::Knight];

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
	pub fn new(piece_index: usize) -> Self {
		match piece_index {
			0 => Piece::King,
			1 => Piece::Pawn,
			2 => Piece::Knight,
			3 => Piece::Bishop,
			4 => Piece::Rook,
			5 => Piece::Queen,
			_ => panic!(),
		}
	}

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
