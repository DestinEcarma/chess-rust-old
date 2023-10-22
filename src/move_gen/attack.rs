use super::MoveGenerator;

use crate::{
	bitboard::{is_occupied, set_bit, Bitboard},
	color::Color,
	piece::Piece,
};

pub type AttackBoards = Vec<Bitboard>;

impl MoveGenerator {
	pub fn get_pawn_attacks(&self, square_index: usize, color: Color) -> Bitboard {
		self.pawn[color.to_index()][square_index]
	}

	pub fn get_slider_attacks(
		&self,
		piece: Piece,
		square_index: usize,
		occupancy: Bitboard,
	) -> Bitboard {
		match piece {
			Piece::Rook => {
				let index = self.rook_magics[square_index].get_index(occupancy);
				self.rook[index]
			}
			Piece::Bishop => {
				let index = self.bishop_magics[square_index].get_index(occupancy);
				self.bishop[index]
			}
			Piece::Queen => {
				let r_index = self.rook_magics[square_index].get_index(occupancy);
				let b_index = self.bishop_magics[square_index].get_index(occupancy);
				self.rook[r_index] ^ self.bishop[b_index]
			}
			_ => panic!("Not a sliding piece: {}", piece.to_full_name()),
		}
	}

	pub fn get_non_slider_attacks(&self, piece: Piece, square_index: usize) -> Bitboard {
		match piece {
			Piece::King => self.king[square_index],
			Piece::Knight => self.knight[square_index],
			_ => panic!("Not a king or a knight: {piece}"),
		}
	}

	pub fn xray_attack(
		&self,
		piece: Piece,
		square_index: usize,
		blockers: Bitboard,
		occupancy: Bitboard,
	) -> (Bitboard, Bitboard) {
		let attack = self.get_slider_attacks(piece, square_index, occupancy);
		let blockers = blockers & attack;

		if blockers == 0 {
			return (blockers, attack);
		} else {
			let xray = attack ^ self.get_slider_attacks(piece, square_index, occupancy ^ blockers);

			return (xray, attack);
		}
	}

	pub fn rook_attack(square_index: u8, occupancy: Bitboard) -> Bitboard {
		const OFFSETS: [i8; 4] = [1, 8, -1, -8];

		let mut attack = 0;

		let num_to_edge = MoveGenerator::rook_num_to_edge(square_index);

		for direction in 0..4 {
			let offset = OFFSETS[direction];

			for n in 1..=num_to_edge[direction] as i8 {
				let target_square = (square_index as i8 + offset * n) as u8;
				set_bit(&mut attack, target_square);

				if is_occupied(occupancy, target_square) {
					break;
				}
			}
		}

		attack
	}

	pub fn bishop_attack(square_index: u8, occupancy: Bitboard) -> Bitboard {
		const OFFSETS: [i8; 4] = [7, 9, -7, -9];

		let mut attack = 0;

		let num_to_edge = MoveGenerator::bishop_num_to_edge(square_index);

		for direction in 0..4 {
			let offset = OFFSETS[direction];

			for n in 1..=num_to_edge[direction] as i8 {
				let target_square = (square_index as i8 + offset * n) as u8;
				set_bit(&mut attack, target_square);

				if is_occupied(occupancy, target_square) {
					break;
				}
			}
		}

		attack
	}

	pub fn rook_attack_boards(square_index: u8, blockers: &[Bitboard]) -> AttackBoards {
		let mut bb_attack_boards = Vec::new();

		for occupancy in blockers {
			bb_attack_boards.push(MoveGenerator::rook_attack(square_index, *occupancy))
		}

		bb_attack_boards
	}

	pub fn bishop_attack_boards(square_index: u8, blockers: &[Bitboard]) -> AttackBoards {
		let mut bb_attack_boards = Vec::new();

		for occupancy in blockers {
			bb_attack_boards.push(MoveGenerator::bishop_attack(square_index, *occupancy));
		}

		bb_attack_boards
	}
}
