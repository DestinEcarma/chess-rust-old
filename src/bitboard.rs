use crate::board::Board;

pub type Bitboard = u64;

pub fn set_bit(bitboard: &mut Bitboard, square_index: u8) {
	*bitboard |= 1u64 << square_index
}

pub fn clear_bit(bitboard: &mut Bitboard, square_index: u8) {
	*bitboard &= !(1u64 << square_index)
}

pub fn is_occupied(bitboard: Bitboard, square_index: u8) -> bool {
	bitboard & (1u64 << square_index) != 0
}

pub fn count_bits(bitboard: Bitboard) -> u8 {
	bitboard.count_ones() as u8
}

pub fn get_lsb_index(bitboard: Bitboard) -> u8 {
	bitboard.trailing_zeros() as u8
}

pub fn get_lsb_bitboard(bitboard: Bitboard) -> Bitboard {
	bitboard & !(bitboard - 1)
}

pub fn pop_lsb(bitboard: &mut Bitboard) -> u8 {
	let lsb_index = get_lsb_index(*bitboard);
	*bitboard &= *bitboard - 1;
	lsb_index
}

pub fn pop_lsb_to_bitboard(bitboard: &mut Bitboard) -> Bitboard {
	let least_bit = get_lsb_bitboard(*bitboard);
	*bitboard &= !least_bit;

	least_bit
}

pub fn get_bit_indices(mut bitboard: Bitboard) -> Vec<u8> {
	let mut indices = Vec::new();

	while bitboard != 0 {
		indices.push(pop_lsb(&mut bitboard))
	}

	indices
}

pub fn print_bitboard(bitboard: Bitboard, name: Option<&str>) {
	if let Some(_name) = name {
		println!(" ↓ {_name}\n")
	}

	let mut board = String::new();

	for rank in (0..8).rev() {
		for file in 0..8 {
			let square_index = Board::to_square_index(rank, file);

			board += if is_occupied(bitboard, square_index) {
				" 1 "
			} else {
				" 0 "
			};
		}

		board += "\n"
	}

	println!("{board}");
	println!(" ↑ Bitboard: {bitboard}\n");
}

pub fn print_board_indices() {
	let mut board = String::new();

	for rank in (0..8).rev() {
		for file in 0..8 {
			let square_index = Board::to_square_index(rank, file);

			board += &format!(" {:02} ", square_index);
		}

		board += "\n"
	}

	println!("{board}");
}
