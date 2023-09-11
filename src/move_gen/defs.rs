use std::fmt::Display;

use super::MoveGenerator;

use crate::{
	bitboard::{pop_lsb, print_bitboard, Bitboard},
	board::Board,
	color::Color,
	notation::Notation,
	piece::{Piece, PROMOTION_PIECES},
};

const PAWN_PROMOTION_RANK: [Bitboard; 2] = [0xff00000000000000, 0x00000000000000ff];

pub struct Shift;
impl Shift {
	const PIECE_MOVED: usize = 0;
	const START_SQUARE: usize = 3;
	const TARGET_SQUARE: usize = 9;
	const MOVE_TYPE: usize = 15;
	const PROMOTION: usize = 18;
	const CASTLE_RIGHT: usize = 21;
	const PREV_ENPASSANT: usize = 25;
	const PREV_CASTLE_RIGHTS: usize = 31;
	const PIECE_CAPTURED: usize = 35;
}

#[derive(Clone, Copy)]
pub struct Move {
	data: u64,
}

impl Display for Move {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		let square_index = self.get_start_square() as usize;
		let target_square = self.get_target_square() as usize;
		let promotion_piece = Piece::new(self.get_promotion());

		if promotion_piece != Piece::King {
			write!(
				f,
				"{}{}{}",
				Notation::from(square_index),
				Notation::from(target_square),
				promotion_piece
			)
		} else {
			write!(
				f,
				"{}{}",
				Notation::from(square_index),
				Notation::from(target_square)
			)
		}
	}
}

impl Move {
	const PIECE_MOVED_MASK: u64 = 0x0000000007;
	const START_SQUARE_MASK: u64 = 0x00000001f8;
	const TARGET_SQUARE_MASK: u64 = 0x0000007e00;
	const MOVE_TYPE_MASK: u64 = 0x0000038000;
	const PROMOTION_MASK: u64 = 0x00001c0000;
	const CASTLE_RIGHT_MASK: u64 = 0x0001e00000;
	const PREV_ENPASSANT_MASK: u64 = 0x007e000000;
	const PREV_CASTLE_RIGHTS_MASK: u64 = 0x0780000000;
	const PIECE_CAPTURED_MASK: u64 = 0x3800000000;

	pub fn new(data: u64) -> Self {
		Move { data }
	}

	pub fn set_prev_enpassant(&mut self, enpassant: Option<usize>) {
		if let Some(ep) = enpassant {
			self.data |= (ep as u64) << Shift::PREV_ENPASSANT;
		}
	}

	pub fn set_prev_castle_rights(&mut self, castle_rights: usize) {
		self.data |= (castle_rights as u64) << Shift::PREV_CASTLE_RIGHTS;
	}

	pub fn set_piece_captured(&mut self, piece: usize) {
		self.data |= (piece as u64) << Shift::PIECE_CAPTURED;
	}

	pub fn get_moved_piece(&self) -> usize {
		(self.data & Move::PIECE_MOVED_MASK) as usize
	}

	pub fn get_start_square(&self) -> u8 {
		((self.data & Move::START_SQUARE_MASK) >> Shift::START_SQUARE) as u8
	}

	pub fn get_target_square(&self) -> u8 {
		((self.data & Move::TARGET_SQUARE_MASK) >> Shift::TARGET_SQUARE) as u8
	}

	pub fn get_move_type(&self) -> usize {
		((self.data & Move::MOVE_TYPE_MASK) >> Shift::MOVE_TYPE) as usize
	}

	pub fn get_promotion(&self) -> usize {
		((self.data & Move::PROMOTION_MASK) >> Shift::PROMOTION) as usize
	}

	pub fn get_castle_right(&self) -> usize {
		((self.data & Move::CASTLE_RIGHT_MASK) >> Shift::CASTLE_RIGHT) as usize
	}

	pub fn get_prev_enpassant(&self) -> Option<usize> {
		let ep = ((self.data & Move::PREV_ENPASSANT_MASK) >> Shift::PREV_ENPASSANT) as usize;

		if ep != 0 {
			return Some(ep);
		}

		return None;
	}

	pub fn get_prev_castle_rights(&self) -> usize {
		((self.data & Move::PREV_CASTLE_RIGHTS_MASK) >> Shift::PREV_CASTLE_RIGHTS) as usize
	}

	pub fn get_piece_captured(&self) -> Option<usize> {
		let piece = ((self.data & Move::PIECE_CAPTURED_MASK) >> Shift::PIECE_CAPTURED) as usize;

		if piece != 0 {
			return Some(piece);
		}

		return None;
	}
}

impl MoveGenerator {
	pub fn add_move(
		&self,
		board: &Board,
		piece: Piece,
		square_index: u8,
		bb_moves: Bitboard,
		list: &mut Vec<Move>,
	) {
		let mut bb_moves = bb_moves;

		let color = board.get_color();
		let promotion_rank = PAWN_PROMOTION_RANK[color.to_index()];
		let is_pawn = piece == Piece::Pawn;
		let is_king = piece == Piece::King;

		while bb_moves > 0 {
			let target_square = pop_lsb(&mut bb_moves) as usize;

			let mut move_type = 0;
			let enpassant = match board.enpassant {
				Some(ep) => {
					if is_pawn && (ep == target_square) {
						1
					} else {
						0
					}
				}
				None => 0,
			};

			let promotion = is_pawn && ((self.square_bit[target_square] & promotion_rank) != 0);

			let direction = square_index as i8 - target_square as i8;
			let castle = is_king && direction.abs() == 2;

			move_type |= enpassant;
			move_type |= (promotion as u64) << 1;
			move_type |= (castle as u64) << 2;

			let mut data = 0;

			data |= (piece.to_index() as u64) << Shift::PIECE_MOVED;
			data |= (square_index as u64) << Shift::START_SQUARE;
			data |= (target_square as u64) << Shift::TARGET_SQUARE;
			data |= move_type << Shift::MOVE_TYPE;

			if castle {
				let mut castle_right;

				if color == Color::White {
					if direction > 0 {
						castle_right = 0b0001;
					} else {
						castle_right = 0b0010;
					}
				} else {
					if direction > 0 {
						castle_right = 0b0100;
					} else {
						castle_right = 0b1000;
					}
				}

				data |= castle_right << Shift::CASTLE_RIGHT;
			}

			if !promotion {
				list.push(Move::new(data));
			} else {
				for piece in PROMOTION_PIECES {
					let promotion = (piece.to_index() << Shift::PROMOTION) as u64;

					list.push(Move::new(data | promotion))
				}
			}
		}
	}
}
