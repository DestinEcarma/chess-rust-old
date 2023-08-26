use super::MoveGenerator;

use crate::{
	bitboard::{get_lsb_index, pop_lsb, pop_lsb_to_bitboard, Bitboard},
	board::Board,
	piece::Piece,
};

impl MoveGenerator {
	fn reset(&mut self) {
		self.check = false;
		self.double_check = false;

		for pin_ray in &mut self.pin_rays {
			*pin_ray = 0;
		}

		self.check_ray = 0;
		self.bb_attack = 0;
	}

	pub fn isolate_attack_pin_checks(&self, mut attack: Bitboard, square_index: usize) -> Bitboard {
		if self.check {
			attack &= self.check_ray;
		}

		let pin_ray = self.pin_rays[square_index];

		if pin_ray != 0 {
			attack &= pin_ray;
		}

		attack
	}

	fn find_pins_checks(&mut self, board: &Board, piece: Piece) {
		let color = board.get_color();
		let inactive = !color;

		let bb_ally_king = board.get_bitboard(Piece::King, color);
		let king_square = get_lsb_index(bb_ally_king) as usize;

		let bb_occupancy = board.get_occupancy();
		let bb_ally_pieces = board.get_allys(color);

		let bb_opp_queens = board.get_bitboard(Piece::Queen, inactive);
		let bb_opp_pieces = board.get_bitboard(piece, inactive);
		let bb_attacking_pieces = bb_opp_queens | bb_opp_pieces;

		let attack = self.get_slider_attacks(piece, king_square, bb_occupancy);

		//* Checks */
		if bb_attacking_pieces > 0 {
			let mut bb_attackers = attack & bb_attacking_pieces;

			while bb_attackers > 0 && !self.double_check {
				self.double_check = self.check;
				self.check = true;

				let square_index = pop_lsb(&mut bb_attackers) as usize;
				let bb_square = Board::square_index_to_bitboard(square_index as u8);

				let _attack = self.get_slider_attacks(piece, square_index, bb_occupancy);

				self.check_ray |= attack & _attack | bb_square;
			}
		}

		let bb_blockers = bb_ally_pieces & attack;

		//* Pins */
		if !self.check && bb_blockers > 0 {
			let xray_attack =
				self.get_slider_attacks(piece, king_square, bb_occupancy ^ bb_blockers);
			let xray = attack ^ xray_attack;

			let mut bb_attackers = xray & bb_attacking_pieces;

			while bb_attackers > 0 {
				let square_index = pop_lsb(&mut bb_attackers) as usize;
				let bb_square = Board::square_index_to_bitboard(square_index as u8);

				let (_xray, _xray_attack) =
					self.xray_attack(piece, square_index, bb_ally_pieces, bb_occupancy);

				let pin_ray = xray_attack & _xray_attack | bb_square;
				let pinned_square = get_lsb_index(pin_ray & bb_blockers) as usize;

				self.pin_rays[pinned_square] |= pin_ray;
			}
		}
	}

	pub fn calculate_attack_mask(&mut self, board: &Board) {
		self.reset();

		self.find_pins_checks(&board, Piece::Rook);
		self.find_pins_checks(&board, Piece::Bishop);

		let color = board.get_color();
		let inactive = !color;

		let bb_ally_king = board.get_bitboard(Piece::King, color);

		//* King attacks */
		let piece = Piece::King;
		let bb_opp_king = board.get_bitboard(piece, inactive);
		self.bb_attack |= self.get_non_slider_attacks(piece, get_lsb_index(bb_opp_king) as usize);

		//* Pawn attacks */
		let piece = Piece::King;
		let mut bb_opp_pawns = board.get_bitboard(piece, inactive);

		let mut is_pawn_check = false;

		while bb_opp_pawns > 0 {
			let square_bit = pop_lsb_to_bitboard(&mut bb_opp_pawns);
			let square_index = get_lsb_index(square_bit) as usize;
			let capture_mask = self.get_pawn_attacks(square_index, inactive);

			self.bb_attack |= capture_mask;

			let in_check = capture_mask & bb_ally_king == bb_ally_king;

			if !is_pawn_check && in_check {
				is_pawn_check = true;

				self.double_check = self.check;
				self.check = true;
				self.check_ray |= square_bit;
			}
		}

		//* Knight attacks */
		let piece = Piece::Knight;
		let mut bb_opp_knight = board.get_bitboard(piece, inactive);

		let mut is_knight_check = false;

		while bb_opp_pawns > 0 {
			let square_bit = pop_lsb_to_bitboard(&mut bb_opp_knight);
			let square_index = get_lsb_index(square_bit) as usize;
			let jump_mask = self.get_non_slider_attacks(piece, square_index);

			self.bb_attack |= jump_mask;

			let in_check = jump_mask & bb_ally_king == bb_ally_king;

			if !is_knight_check && in_check {
				is_knight_check = true;

				self.double_check = self.check;
				self.check = true;
				self.check_ray |= square_bit;
			}
		}

		//* Sliding piece attacks */
		let bb_no_king_occ = board.get_occupancy() & !bb_ally_king;
		let bb_opp_queens = board.get_bitboard(Piece::Queen, inactive);

		let piece = Piece::Rook;
		let bb_opp_rook = board.get_bitboard(piece, inactive);

		let mut bb_attackers = bb_opp_queens | bb_opp_rook;

		while bb_attackers > 0 {
			let square_index = pop_lsb(&mut bb_attackers) as usize;
			let attack = self.get_slider_attacks(piece, square_index, bb_no_king_occ);

			self.bb_attack |= attack;
		}

		let piece = Piece::Bishop;
		let bb_opp_bishop = board.get_bitboard(piece, inactive);

		let mut bb_attackers = bb_opp_queens | bb_opp_bishop;

		while bb_attackers > 0 {
			let square_index = pop_lsb(&mut bb_attackers) as usize;
			let attack = self.get_slider_attacks(piece, square_index, bb_no_king_occ);

			self.bb_attack |= attack;
		}
	}
}
