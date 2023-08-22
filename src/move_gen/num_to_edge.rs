use crate::board::Board;

use super::MoveGen;

impl MoveGen {
	pub fn rook_num_to_edge(square_index: u8) -> [u8; 4] {
		let (rank, file) = Board::square_to_rank_file(square_index);
		let rank = rank as i8;
		let file = file as i8;

		[
			u8::max((7 - file) as u8, 0),
			u8::max((7 - rank) as u8, 0),
			u8::max(file as u8, 0),
			u8::max(rank as u8, 0),
		]
	}

	pub fn bishop_num_to_edge(square_index: u8) -> [u8; 4] {
		let (rank, file) = Board::square_to_rank_file(square_index);
		let rank = rank as i8;
		let file = file as i8;

		let num_east = i8::max(7 - file, 0) as u8;
		let num_north = i8::max(7 - rank, 0) as u8;
		let num_west = i8::max(file, 0) as u8;
		let num_south = i8::max(rank, 0) as u8;

		[
			u8::min(num_north, num_west),
			u8::min(num_north, num_east),
			u8::min(num_south, num_east),
			u8::min(num_south, num_west),
		]
	}
}
