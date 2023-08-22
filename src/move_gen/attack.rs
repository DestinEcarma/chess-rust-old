use crate::{bitboard::Bitboard, board::Board};

use super::MoveGen;

impl MoveGen {
	pub fn rook_attack(square_index: u8, occupancy: Bitboard) -> Bitboard {
		let attack = 0;

		let (rank, file) = Board::square_to_rank_file(square_index);
		
		attack
	}
}