use crate::{
	bitboard::{set_bit, Bitboard},
	color::Color,
	error::Error,
	notation::{self, Notation},
	piece::Piece,
};
use std::{
	fmt::{Display, Formatter, Result},
	str::FromStr,
};

#[allow(dead_code)]
pub struct Board {
	pieces: [Bitboard; 6],
	colors: [Bitboard; 2],

	turn: Color,

	pub caslting_rights: u8,
	pub enpassant: Option<u8>,
}

impl Default for Board {
	fn default() -> Self {
		Board::from_str(&"3K4/3PP3/3q4/6b1/8/8/8/8 w - - 0 1").unwrap()
	}
}

impl Display for Board {
	fn fmt(&self, f: &mut Formatter<'_>) -> Result {
		let board = String::from("Test");

		write!(f, "{}", board)
	}
}

impl FromStr for Board {
	type Err = Error;

	fn from_str(value: &str) -> std::result::Result<Self, Self::Err> {
		let mut rank = 7u8;
		let mut file = 0u8;
		let mut board = Board::new();

		let tokens: Vec<&str> = value.split(' ').collect();

		if tokens.len() < 4 {
			return Err(Error::InvalidFen {
				fen: value.to_string(),
			});
		}

		let pieces = tokens[0];
		let turn = tokens[1];
		let castle_right = tokens[2];
		let enpassant = tokens[3];

		for char in pieces.chars() {
			match char {
				'/' => {
					rank -= 1;
					file = 0;
				}
				'1' | '2' | '3' | '4' | '5' | '6' | '7' | '8' => {
					file += char.to_digit(10).unwrap() as u8
				}
				'K' | 'P' | 'N' | 'B' | 'R' | 'Q' | 'k' | 'p' | 'n' | 'b' | 'r' | 'q' => {
					let square_index = Board::to_square_index(rank, file);
					let color = Color::from(char);
					let piece = Piece::from(char);

					set_bit(&mut board.pieces[piece.to_index()], square_index);
					set_bit(&mut board.colors[color.to_index()], square_index);
					file += 1;
				}
				_ => {
					return Err(Error::InvalidFen {
						fen: value.to_string(),
					})
				}
			}
		}

		match turn {
			"w" | "W" => board.turn = Color::White,
			"b" | "B" => board.turn = Color::Black,
			_ => {
				return Err(Error::InvalidFen {
					fen: value.to_string(),
				})
			}
		}

		for char in castle_right.chars() {
			match char {
				'Q' => board.caslting_rights |= 1u8 << 0,
				'K' => board.caslting_rights |= 1u8 << 1,
				'q' => board.caslting_rights |= 1u8 << 2,
				'k' => board.caslting_rights |= 1u8 << 3,
				'-' => continue,
				_ => {
					return Err(Error::InvalidFen {
						fen: value.to_string(),
					})
				}
			}
		}

		if let Ok(notation) = Notation::from_str(&enpassant) {
			board.enpassant = Some(notation as u8)
		}

		Ok(board)
	}
}

impl Board {
	pub fn new() -> Board {
		Board {
			pieces: [0; 6],
			colors: [0; 2],
			turn: Color::White,
			caslting_rights: 0,
			enpassant: None,
		}
	}

	pub fn to_square_index(rank: u8, file: u8) -> u8 {
		rank * 8 + file
	}

	pub fn square_to_rank_file(square_index: u8) -> (u8, u8) {
		let rank = square_index / 8;
		let file = square_index - rank * 8;

		(rank, file)
	}

	pub fn square_index_to_bitboard(square_index: u8) -> Bitboard {
		let mut bitboard = 0;
		set_bit(&mut bitboard, square_index);

		bitboard
	}

	pub fn get_color(&self) -> Color {
		self.turn
	}

	pub fn get_bitboard(&self, piece: Piece, color: Color) -> Bitboard {
		self.pieces[piece.to_index()] & self.colors[color.to_index()]
	}

	pub fn get_occupancy(&self) -> Bitboard {
		self.colors[Color::White.to_index()] | self.colors[Color::Black.to_index()]
	}

	pub fn get_allys(&self, color: Color) -> Bitboard {
		self.colors[color.to_index()]
	}
}
