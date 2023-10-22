use crate::{
	bitboard::{clear_bit, is_occupied, set_bit, Bitboard},
	color::Color,
	error::Error,
	move_gen::{defs::Move, PAWN_PUSH_DIRECTION},
	notation::Notation,
	piece::{Piece, ALL_PIECES},
};
use std::{
	fmt::{Display, Formatter, Result},
	str::FromStr,
};

const CASTLE_ROOK_START: [u8; 4] = [00, 07, 56, 63];
const CASTLE_ROOK_TARGET: [u8; 4] = [03, 05, 59, 61];

#[allow(dead_code)]
pub struct Board {
	pieces: [Bitboard; 6],
	colors: [Bitboard; 2],

	color: Color,

	pub castle_rights: usize,
	pub enpassant: Option<usize>,

	move_history: Vec<Move>,
}

impl Default for Board {
	fn default() -> Self {
		Board::from_str(&"rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1").unwrap()
	}
}

impl Display for Board {
	fn fmt(&self, f: &mut Formatter<'_>) -> Result {
		let mut board = String::from("   +———+———+———+———+———+———+———+———+\n");

		for rank in (0..8).rev() {
			for file in 0..8 {
				if file == 0 {
					board += &format!(" {} ", rank + 1)
				};

				let square_index = Board::to_square_index(rank, file);
				let piece = self.get_piece_from_square(square_index);

				if let Some(piece) = piece {
					let is_white = is_occupied(self.colors[0], square_index);
					let color = if is_white { Color::White } else { Color::Black };
					let str_piece = Piece::new(piece).to_string(color);

					board += &format!("| {} ", str_piece);
				} else {
					board += "|   ";
				}

				if file == 7 {
					board += "|";
				}
			}

			board += "\n   +———+———+———+———+———+———+———+———+\n";
		}

		board += "     a   b   c   d   e   f   g   h\n";
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
			"w" | "W" => board.color = Color::White,
			"b" | "B" => board.color = Color::Black,
			_ => {
				return Err(Error::InvalidFen {
					fen: value.to_string(),
				})
			}
		}

		for char in castle_right.chars() {
			match char {
				'Q' => board.castle_rights |= 0b0001,
				'K' => board.castle_rights |= 0b0010,
				'q' => board.castle_rights |= 0b0100,
				'k' => board.castle_rights |= 0b1000,
				'-' => continue,
				_ => {
					return Err(Error::InvalidFen {
						fen: value.to_string(),
					})
				}
			}
		}

		if let Ok(notation) = Notation::from_str(&enpassant) {
			board.enpassant = Some(notation as usize)
		}

		Ok(board)
	}
}

impl Board {
	pub fn new() -> Self {
		Self {
			pieces: [0; 6],
			colors: [0; 2],
			color: Color::White,
			castle_rights: 0,
			enpassant: None,
			move_history: Vec::new(),
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

	pub fn get_color(&self) -> Color {
		self.color
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

	fn get_piece_from_square(&self, square_index: u8) -> Option<usize> {
		let mut piece_on_square = None;

		for piece in ALL_PIECES {
			let piece_index = piece.to_index();

			if is_occupied(self.pieces[piece_index], square_index) {
				piece_on_square = Some(piece_index);
				break;
			}
		}

		piece_on_square
	}

	fn _move_bit(&mut self, piece: usize, color: usize, start_square: u8, target_square: u8) {
		let bb_piece = &mut self.pieces[piece];
		let bb_ally = &mut self.colors[color];

		clear_bit(bb_piece, start_square);
		set_bit(bb_piece, target_square);

		clear_bit(bb_ally, start_square);
		set_bit(bb_ally, target_square);
	}

	fn _capture_bit(&mut self, captured: usize, color: usize, target_square: u8) {
		clear_bit(&mut self.pieces[captured], target_square);
		clear_bit(&mut self.colors[color], target_square);
	}

	fn _undo_move_bit(&mut self, piece: usize, color: usize, start_square: u8, target_square: u8) {
		self._move_bit(piece, color, target_square, start_square);
	}

	fn _undo_capture_bit(&mut self, captured: usize, color: usize, target_square: u8) {
		set_bit(&mut self.pieces[captured], target_square);
		set_bit(&mut self.colors[color], target_square);
	}

	fn _promotion(&mut self, piece: usize, target_square: u8) {
		clear_bit(&mut self.pieces[Piece::Pawn.to_index()], target_square);
		set_bit(&mut self.pieces[piece], target_square);
	}

	fn _undo_promotion(&mut self, piece: usize, target_square: u8) {
		clear_bit(&mut self.pieces[piece], target_square);
	}

	pub fn make_move(&mut self, mut to_move: Move) {
		let color = self.color.to_index();
		let inactive = (!self.color).to_index();

		let piece_moved = to_move.get_moved_piece();
		let start_square = to_move.get_start_square();
		let target_square = to_move.get_target_square();
		let piece_captured = self.get_piece_from_square(target_square);

		to_move.set_prev_enpassant(self.enpassant);
		to_move.set_prev_castle_rights(self.castle_rights);
		self.enpassant = None;

		if let Some(piece) = piece_captured {
			self._capture_bit(piece, inactive, target_square);
			to_move.set_piece_captured(piece);

			// Is rook captured
			if piece == 4 {
				let castle_index = CASTLE_ROOK_START.iter().position(|&i| i == target_square);

				if let Some(castle_index) = castle_index {
					self.castle_rights &= !(1 << castle_index)
				}
			}
		}

		self._move_bit(piece_moved, color, start_square, target_square);

		let i8_start_square = start_square as i8;
		let i8_target_square = target_square as i8;

		let move_type = to_move.get_move_type();

		match piece_moved {
			//King
			0 => {
				// If a king moved then clear color castling rights
				match self.color {
					Color::White => self.castle_rights &= 0b1100,
					Color::Black => self.castle_rights &= 0b0011,
				}
			}
			// Pawn
			1 => {
				// Set enpassant if move is a double push
				if i8::abs(i8_start_square - i8_target_square) == 16 {
					self.enpassant = Some((i8_target_square - PAWN_PUSH_DIRECTION[color]) as usize);
				}
			}

			// Rook
			4 => {
				let castle_index = CASTLE_ROOK_START.iter().position(|&i| i == start_square);

				if let Some(castle_index) = castle_index {
					self.castle_rights &= !(1 << castle_index)
				}
			}
			_ => {}
		}

		match move_type {
			// Enpassant
			0b0001 => {
				let pawn_square = i8_target_square - PAWN_PUSH_DIRECTION[color];
				let piece_captured = piece_moved;

				self._capture_bit(piece_captured, inactive, pawn_square as u8);
			}
			// Promotion
			0b0010 => {
				let piece = to_move.get_promotion();

				self._promotion(piece, target_square);
			}
			// Castle
			0b0100 => {
				let castle_right = to_move.get_castle_right();
				let castle_index = castle_right.trailing_zeros() as usize;

				let rook_start = CASTLE_ROOK_START[castle_index];
				let rook_target = CASTLE_ROOK_TARGET[castle_index];

				self._move_bit(Piece::Rook.to_index(), color, rook_start, rook_target);
			}
			_ => {}
		}

		self.color = !self.color;
		self.move_history.push(to_move);
	}

	pub fn undo_move(&mut self) -> bool {
		if let Some(prev_move) = self.move_history.pop() {
			let color = (!self.color).to_index();
			let inactive = self.color.to_index();

			let piece_moved = prev_move.get_moved_piece();
			let start_square = prev_move.get_start_square();
			let target_square = prev_move.get_target_square();
			let piece_captured = prev_move.get_piece_captured();

			self._undo_move_bit(piece_moved, color, start_square, target_square);

			let i8_target_square = target_square as i8;
			let move_type = prev_move.get_move_type();

			match move_type {
				// Enpassant
				0b0001 => {
					let pawn_square = i8_target_square - PAWN_PUSH_DIRECTION[color];
					let piece_captured = piece_moved;

					self._undo_capture_bit(piece_captured, inactive, pawn_square as u8);
				}
				// Promotion
				0b0010 => {
					let piece = prev_move.get_promotion();

					self._undo_promotion(piece, target_square);
				}
				// Castle
				0b0100 => {
					let castle_right = prev_move.get_castle_right();
					let castle_index = castle_right.trailing_zeros() as usize;

					let rook_start = CASTLE_ROOK_START[castle_index];
					let rook_target = CASTLE_ROOK_TARGET[castle_index];

					self._undo_move_bit(Piece::Rook.to_index(), color, rook_start, rook_target);
				}
				_ => {}
			}

			if let Some(piece) = piece_captured {
				self._undo_capture_bit(piece, inactive, target_square);
			}

			self.color = !self.color;
			self.enpassant = prev_move.get_prev_enpassant();
			self.castle_rights = prev_move.get_prev_castle_rights();

			return true;
		}

		return false;
	}
}
