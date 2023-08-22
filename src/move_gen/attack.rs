use crate::{
	bitboard::{is_occupied, set_bit, Bitboard},
	board::Board,
};

use super::MoveGen;

impl MoveGen {
	pub fn rook_attack(square_index: u8, occupancy: Bitboard) -> Bitboard {
		let mut attack = 0;

		let offsets = [1, 8, -1, -8];
		let num_to_edge = MoveGen::rook_num_to_edge(square_index);

		for direction in 0..4 {
			let offset = offsets[direction];

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
		let mut attack = 0;

		let offsets = [7, 9, -7, -9];
		let num_to_edge = MoveGen::bishop_num_to_edge(square_index);

		for direction in 0..4 {
			let offset = offsets[direction];

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
}
