use super::{num_to_edge, MoveGen};

use crate::{
	bitboard::{set_bit, Bitboard},
	board::Board,
};

const EDGE: Bitboard = 0xff818181818181ff;

impl MoveGen {
	pub fn rook_mask(square_index: u8) -> Bitboard {
		let mut mask = 0;

		let offsets = [1, 8, -1, -8];
		let num_to_edge = MoveGen::rook_num_to_edge(square_index);

		for direction in 0..4 {
			let offset = offsets[direction];

			for n in 1..=num_to_edge[direction] as i8 {
				set_bit(&mut mask, (square_index as i8 + offset * n) as u8)
			}
		}

		mask & !EDGE
	}

	pub fn bishop_mask(square_index: u8) -> Bitboard {
		let mut mask = 0;

		let offsets = [7, 9, -7, -9];
		let num_to_edge = MoveGen::bishop_num_to_edge(square_index);

		for direction in 0..4 {
			let offset = offsets[direction];

			for n in 1..=num_to_edge[direction] as i8 {
				set_bit(&mut mask, (square_index as i8 + offset * n) as u8);
			}
		}

		mask & !EDGE
	}
}
