use crate::{
	bitboard::{set_bit, Bitboard},
	color::Color,
	error::Error,
	piece::Piece,
};
use std::{
	fmt::{Display, Formatter, Result},
	str::FromStr,
};

#[allow(dead_code)]
pub struct Board {
	pub pieces: [Bitboard; 6],
	pub colors: [Bitboard; 2],

	pub turn: Color,

	pub caslting_rights: u8,
	pub enpassant: Option<u8>,
}

impl Default for Board {
	fn default() -> Self {
		Board::from_str("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1").unwrap()
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
		// let enpassant = tokens[3];

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
					let square_index = rank * 8 + file;
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
				_ => {
					return Err(Error::InvalidFen {
						fen: value.to_string(),
					})
				}
			}
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

	pub fn get_bitboard(&self, piece: Piece, color: Color) -> Bitboard {
		self.pieces[piece.to_index()] & self.colors[color.to_index()]
	}
}

pub fn print_board_indices() {
	let mut board = String::new();

	for rank in (0..8).rev() {
		for file in 0..8 {
			let square_index = rank * 8 + file;

			board += &format!(" {:02} ", square_index);
		}

		board += "\n"
	}

	println!("{board}");
}
