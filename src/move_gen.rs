mod attack;
mod defs;
mod extra;
mod init;
mod mask;
mod num_to_edge;

use std::io::empty;

use self::defs::Move;

use crate::{
	bitboard::{
		self, get_lsb_index, is_occupied, pop_lsb, pop_lsb_to_bitboard, print_bitboard, set_bit,
		Bitboard,
	},
	board::Board,
	color::Color,
	magic::Magic,
	piece::Piece, notation::{self, Notation},
};

const PAWN_PUSH_DIRECTION: [i8; 2] = [8, -8];
const PAWN_DOUBLE_PUSH_MASK: [u64; 2] = [0x00000000ff000000, 0x000000ff00000000];
const PAWN_PROMOTION_MASK: [u64; 2] = [0x00000000000000ff, 0xff00000000000000];

pub struct MoveGenerator {
	king: [Bitboard; 64],
	pawn: [[Bitboard; 64]; 2],
	knight: [Bitboard; 64],
	rook: Vec<Bitboard>,
	bishop: Vec<Bitboard>,
	rook_magics: [Magic; 64],
	bishop_magics: [Magic; 64],

	check: bool,
	double_check: bool,

	pub pin_rays: [Bitboard; 64],
	pub check_ray: Bitboard,
	pub bb_attack: Bitboard,
}

impl Default for MoveGenerator {
	fn default() -> Self {
		let mut move_gen = Self {
			king: [0; 64],
			pawn: [[0; 64]; 2],
			knight: [0; 64],
			rook: vec![0; 102_400],
			bishop: vec![0; 5_248],
			rook_magics: [Magic::default(); 64],
			bishop_magics: [Magic::default(); 64],

			check: false,
			double_check: false,

			pin_rays: [0; 64],
			check_ray: 0,
			bb_attack: 0,
		};
		move_gen.init();

		move_gen
	}
}

impl MoveGenerator {
	pub fn piece(&self, board: &Board, piece: Piece, list: &mut Vec<Move>) {
		let color = board.get_color();
		let bb_occupancy = board.get_occupancy();

		let bb_empty = !bb_occupancy;
		let bb_ally_pieces = board.get_allys(color);
		let bb_opponent_pieces = board.get_allys(!color);

		let mut bb_pieces = board.get_bitboard(piece, color);

		while bb_pieces > 0 {
			let square_index = pop_lsb(&mut bb_pieces) as usize;
			let bb_target = match piece {
				Piece::King | Piece::Knight => self.get_non_slider_attacks(piece, square_index),
				Piece::Queen | Piece::Rook | Piece::Bishop => {
					self.get_slider_attacks(piece, square_index, bb_occupancy)
				}
				_ => panic!("Not a piece: {}", piece.to_full_name()),
			} & !bb_ally_pieces;

			let bb_moves = match piece {
				Piece::King => bb_target & !self.bb_attack,
				_ => self.isolate_attack_pin_checks(bb_target, square_index),
			};

			print_bitboard(
				bb_moves,
				Some(format!("{} -> {}", piece.to_full_name(), Notation::from(square_index)).as_str()),
			)
		}
	}

	pub fn pawns(&self, board: &Board, list: &mut Vec<Move>) {
		let color = board.get_color();

		let bb_empty = !board.get_occupancy();
		let bb_opponent_pieces = board.get_allys(!color);
		let mut bb_pawns = board.get_bitboard(Piece::Pawn, color);

		let color_index = color.to_index();
		let double_push_mask = PAWN_DOUBLE_PUSH_MASK[color_index];
		let direction = PAWN_PUSH_DIRECTION[color_index];
		let rotation_count = (64 + direction) as u32;

		while bb_pawns > 0 {
			let square_index = pop_lsb(&mut bb_pawns) as usize;
			let mut bb_moves = 0;

			//* Pushs */
			{
				let bb_push =
					Board::square_index_to_bitboard((square_index as i8 + direction) as u8);
				let bb_one_step = bb_push & bb_empty;
				let bb_two_step =
					bb_one_step.rotate_left(rotation_count) & bb_empty & double_push_mask;

				bb_moves |= bb_one_step | bb_two_step;
			}

			//* Captures */
			{
				let capture_mask = self.get_pawn_attacks(square_index, color);
				let bb_capture = capture_mask & bb_opponent_pieces;
				let bb_ep_capture = match board.enpassant {
					Some(ep) => capture_mask & Board::square_index_to_bitboard(ep),
					None => 0,
				};

				bb_moves |= bb_capture | bb_ep_capture;
			}

			self.isolate_attack_pin_checks(bb_moves, square_index);

			if bb_moves > 0 {
				//* add_moves */
			}
		}
	}

	pub fn castling(&self, board: &Board, list: &mut Vec<Move>) {
		let turn = board.get_color();
	}
}
