mod attack;
pub mod defs;
mod extra;
mod init;
mod mask;
mod num_to_edge;

use self::defs::Move;

use crate::{
	bitboard::{get_lsb_index, pop_lsb, set_bit, Bitboard},
	board::Board,
	magic::Magic,
	piece::Piece,
};

pub const PAWN_PUSH_DIRECTION: [i8; 2] = [8, -8];
pub const CASTLE_KING_INDEX: [u8; 4] = [02, 06, 58, 62];

const PAWN_DOUBLE_PUSH_RANK: [Bitboard; 2] = [0x00000000ff000000, 0x000000ff00000000];

const CASTLE_RIGHTS: [usize; 2] = [0b0011, 0b1100];
const CASTLE_EMPTY: [Bitboard; 4] = [
	0x000000000000000e,
	0x0000000000000060,
	0x0e00000000000000,
	0x6000000000000000,
];

const CASTLE_ATTACK: [Bitboard; 4] = [
	0x000000000000000c,
	0x0000000000000060,
	0x0c00000000000000,
	0x6000000000000000,
];

pub struct MoveGenerator {
	king: [Bitboard; 64],
	pawn: [[Bitboard; 64]; 2],
	knight: [Bitboard; 64],
	rook: Vec<Bitboard>,
	bishop: Vec<Bitboard>,
	rook_magics: [Magic; 64],
	bishop_magics: [Magic; 64],

	square_bit: [u64; 64],

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

			square_bit: [0; 64],

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
	pub fn all_possible(&mut self, board: &Board) -> Vec<Move> {
		let mut move_list = Vec::new();

		self.calculate_attack_mask(&board);

		self.piece(&board, Piece::King, &mut move_list);
		self.castling(&board, &mut move_list);

		if self.double_check {
			return move_list;
		}

		self.pawns(&board, &mut move_list);
		self.enpassant(&board, &mut move_list);

		self.piece(&board, Piece::Knight, &mut move_list);
		self.piece(&board, Piece::Bishop, &mut move_list);
		self.piece(&board, Piece::Rook, &mut move_list);
		self.piece(&board, Piece::Queen, &mut move_list);

		move_list
	}

	pub fn piece(&self, board: &Board, piece: Piece, list: &mut Vec<Move>) {
		let color = board.get_color();

		let bb_occupancy = board.get_occupancy();
		let bb_ally_pieces = board.get_allys(color);
		let mut bb_pieces = board.get_bitboard(piece, color);

		let is_king = piece == Piece::King;

		while bb_pieces > 0 {
			let square_index = pop_lsb(&mut bb_pieces) as usize;
			let bb_target = match piece {
				Piece::King | Piece::Knight => self.get_non_slider_attacks(piece, square_index),
				Piece::Queen | Piece::Rook | Piece::Bishop => {
					self.get_slider_attacks(piece, square_index, bb_occupancy)
				}
				_ => panic!("Not a piece: {}", piece.to_full_name()),
			} & !bb_ally_pieces;

			let mut bb_moves = bb_target;

			if is_king {
				bb_moves &= !self.bb_attack;
			} else {
				self.isolate_attack_pin_checks(&mut bb_moves, square_index);
			}

			if bb_moves > 0 {
				self.add_move(&board, piece, square_index as u8, bb_moves, list);
			}
		}
	}

	pub fn pawns(&self, board: &Board, list: &mut Vec<Move>) {
		let color = board.get_color();
		let inactive = !color;

		let bb_empty = !board.get_occupancy();

		let bb_opponent_pieces = board.get_allys(inactive);
		let mut bb_ally_pawns = board.get_bitboard(Piece::Pawn, color);

		let color_index = color.to_index();
		let double_push_rank = PAWN_DOUBLE_PUSH_RANK[color_index];
		let direction = PAWN_PUSH_DIRECTION[color_index];
		let rotation_count = (64 + direction) as u32;

		while bb_ally_pawns > 0 {
			let square_index = pop_lsb(&mut bb_ally_pawns) as usize;
			let mut bb_moves = 0;

			//* Pushs */
			{
				let bb_push = self.square_bit[(square_index as i8 + direction) as usize];
				let bb_one_step = bb_push & bb_empty;
				let bb_two_step =
					bb_one_step.rotate_left(rotation_count) & bb_empty & double_push_rank;

				bb_moves |= bb_one_step | bb_two_step;
			}

			//* Captures */
			{
				let capture_mask = self.get_pawn_attacks(square_index, color);
				let bb_capture = capture_mask & bb_opponent_pieces;

				bb_moves |= bb_capture;
			}

			self.isolate_attack_pin_checks(&mut bb_moves, square_index);

			if bb_moves > 0 {
				self.add_move(&board, Piece::Pawn, square_index as u8, bb_moves, list);
			}
		}
	}

	pub fn enpassant(&self, board: &Board, list: &mut Vec<Move>) {
		if board.enpassant == None {
			return;
		}

		let color = board.get_color();
		let inactive = !color;

		let bb_occupancy = board.get_occupancy();

		let bb_ally_king = board.get_bitboard(Piece::King, color);
		let bb_ally_pawns = board.get_bitboard(Piece::Pawn, color);

		let bb_opp_rooks = board.get_bitboard(Piece::Rook, inactive);
		let bb_opp_queens = board.get_bitboard(Piece::Queen, inactive);

		let enpassant_pin_rank = PAWN_DOUBLE_PUSH_RANK[inactive.to_index()];
		let bb_enpassant_pieces = bb_ally_king | bb_opp_rooks | bb_opp_queens;

		let target_square = board.enpassant.unwrap();
		let bb_moves = self.square_bit[target_square];

		let mut bb_pawns = bb_ally_pawns & self.get_pawn_attacks(target_square, inactive);

		if bb_pawns != 0 {
			//* Check if en passant capture is a legal move */
			if (enpassant_pin_rank & bb_ally_king) != 0 && bb_pawns.count_ones() == 1 {
				let square_index = get_lsb_index(bb_pawns) as usize;

				#[rustfmt::skip]
				let (xray, attack) = self.xray_attack(
					Piece::Rook,
					square_index,
					bb_occupancy,
					bb_occupancy
				);

				let xray = xray & enpassant_pin_rank;
				let attack = attack & enpassant_pin_rank;

				let bb_piece_in_attack = attack & bb_enpassant_pieces;
				let bb_piece_in_xray = xray & bb_enpassant_pieces;

				if bb_piece_in_attack != 0 && bb_piece_in_xray != 0 {
					bb_pawns = 0;
				}
			}

			if self.check {
				let pawn_square = target_square as i8 - PAWN_PUSH_DIRECTION[color.to_index()];
				let pawn_bit = self.square_bit[pawn_square as usize];

				if pawn_bit & self.check_ray == 0 {
					bb_pawns = 0;
				}
			}

			while bb_pawns > 0 {
				let square_index = pop_lsb(&mut bb_pawns);

				if self.pin_rays[square_index as usize] != 0 {
					continue;
				}

				self.add_move(&board, Piece::Pawn, square_index, bb_moves, list);
			}
		}
	}

	pub fn castling(&self, board: &Board, list: &mut Vec<Move>) {
		if self.check {
			return;
		}

		let color = board.get_color();

		let bb_empty = !board.get_occupancy();
		let bb_ally_king = board.get_bitboard(Piece::King, color);

		let mut ally_castle_rights =
			(CASTLE_RIGHTS[color.to_index()] & board.castle_rights) as Bitboard;

		let mut bb_moves = 0;

		while ally_castle_rights > 0 {
			let castle_index = pop_lsb(&mut ally_castle_rights) as usize;

			let bb_castle_empty = CASTLE_EMPTY[castle_index];
			if bb_empty & bb_castle_empty != bb_castle_empty {
				continue;
			}

			let bb_caslte_attack = CASTLE_ATTACK[castle_index];
			if bb_caslte_attack & !self.bb_attack != bb_caslte_attack {
				continue;
			}

			set_bit(&mut bb_moves, CASTLE_KING_INDEX[castle_index]);
		}

		if bb_moves > 0 {
			let king_square = get_lsb_index(bb_ally_king);

			self.add_move(&board, Piece::King, king_square, bb_moves, list);
		}
	}
}
