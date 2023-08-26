use super::MoveGenerator;

use crate::bitboard::{print_bitboard, set_bit, Bitboard};

pub type BlockerBoards = Vec<Bitboard>;

impl MoveGenerator {
	pub fn rook_mask(square_index: u8) -> Bitboard {
		const OFFSETS: [i8; 4] = [1, 8, -1, -8];

		let mut mask = 0;

		let num_to_edge = MoveGenerator::rook_num_to_edge(square_index);

		for direction in 0..4 {
			let offset = OFFSETS[direction];

			for n in 0..(i8::max(num_to_edge[direction] as i8 - 1, 0)) {
				set_bit(&mut mask, (square_index as i8 + offset * (n + 1)) as u8);
			}
		}

		mask
	}

	pub fn bishop_mask(square_index: u8) -> Bitboard {
		const OFFSETS: [i8; 4] = [7, 9, -7, -9];

		let mut mask = 0;

		let num_to_edge = MoveGenerator::bishop_num_to_edge(square_index);

		for direction in 0..4 {
			let offset = OFFSETS[direction];

			for n in 0..(i8::max(num_to_edge[direction] as i8 - 1, 0)) {
				set_bit(&mut mask, (square_index as i8 + offset * (n + 1)) as u8);
			}
		}

		mask
	}

	pub fn blocker_boards(mask: Bitboard) -> BlockerBoards {
		let mut bb_blocker_boards = Vec::new();
		let mut current_mask = 0u64;

		// Carry-Rippler
		// https://www.chessprogramming.org/Traversing_Subsets_of_a_Set
		loop {
			bb_blocker_boards.push(current_mask);
			current_mask = current_mask.wrapping_sub(mask) & mask;

			if current_mask == 0 {
				break;
			}
		}

		bb_blocker_boards
	}
}
