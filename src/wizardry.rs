use rand::Rng;

use crate::{
	bitboard::{is_bit_occupied, pop_lsb, set_bit},
	magics::Magic,
};

#[rustfmt::skip]
const BIT_TABLE: [u8; 64] = [
	63, 30, 03, 32, 25, 41, 22, 33,
	15, 50, 42, 13, 11, 53, 19, 34,
	61, 29, 02, 51, 21, 43, 45, 10,
	18, 47, 01, 54, 09, 57, 00, 35,
	62, 31, 40, 04, 49, 05, 52, 26,
	60, 06, 23, 44, 46, 27, 56, 16,
	07, 39, 48, 24, 59, 14, 12, 55,
	38, 28, 58, 20, 37, 17, 36, 08,
];

#[rustfmt::skip]
const LEVEL_BITS: [u8; 64] = [
	12, 11, 11, 11, 11, 11, 11, 12,
	11, 10, 10, 10, 10, 10, 10, 11,
	11, 10, 10, 10, 10, 10, 10, 11,
	11, 10, 10, 10, 10, 10, 10, 11,
	11, 10, 10, 10, 10, 10, 10, 11,
	11, 10, 10, 10, 10, 10, 10, 11,
	11, 10, 10, 10, 10, 10, 10, 11,
	12, 11, 11, 11, 11, 11, 11, 12
];

#[rustfmt::skip]
const DIAGONAL_BITS: [u8; 64] = [
	6, 5, 5, 5, 5, 5, 5, 6,
	5, 5, 5, 5, 5, 5, 5, 5,
	5, 5, 7, 7, 7, 7, 5, 5,
	5, 5, 7, 9, 9, 7, 5, 5,
	5, 5, 7, 9, 9, 7, 5, 5,
	5, 5, 7, 7, 7, 7, 5, 5,
	5, 5, 5, 5, 5, 5, 5, 5,
	6, 5, 5, 5, 5, 5, 5, 6
];

pub fn sliding_attack(target_squares: &[&[u8]], blocker: u64) -> u64 {
	let mut attack: u64 = 0;

	for i in 0..target_squares.len() {
		for target_square in target_squares[i] {
			set_bit(&mut attack, *target_square);

			if is_bit_occupied(blocker, *target_square) {
				break;
			}
		}
	}

	attack
}

fn random_u64_fewbits() -> u64 {
	let mut rng = rand::thread_rng();

	rng.gen::<u64>() & rng.gen::<u64>() & rng.gen::<u64>()
}

fn index_to_uint64(index: usize, bits: u8, mut bitboard: u64) -> u64 {
	let mut result: u64 = 0;

	for i in 0..bits {
		let square_index = pop_lsb(&mut bitboard);

		if index & (1 << i) != 0 {
			result |= 1 << square_index;
		}
	}

	result
}

fn generate_64_magic_numbers(piece_masks: [u64; 64], target_squares: [&[&[u8]]; 64], len: usize) {
	let mut offset = 0;

	for square_index in 0..64 {
		let mask = piece_masks[square_index];
		let squares = target_squares[square_index];

		let bits = mask.count_ones() as u8;
		let permutations = (1u32 << bits) as usize;
		let end = offset + permutations - 1;

		let mut blockers = vec![0u64; permutations];
		let mut attacks = vec![0u64; permutations];
		let mut used_attacks = vec![0u64; len];

		for i in 0..permutations {
			blockers[i] = index_to_uint64(i, bits, mask);
			attacks[i] = sliding_attack(squares, blockers[i]);
		}

		let mut magic: Magic = Default::default();
		let mut found = false;
		let mut attempts = 0;

		magic.mask = mask;
		magic.shift = 64 - bits;
		magic.offset = offset as u64;

		while !found {
			attempts += 1;
			found = true;

			magic.nr = random_u64_fewbits();

			for next in 1..permutations {
				let index = magic.get_index(blockers[next]);

				if used_attacks[index] == 0 {
					let fail_low = index < offset as usize;
					let fail_high = index > end as usize;
					assert!(!fail_low && !fail_high, "Indexing error.");

					used_attacks[index] = attacks[next];
				} else {
					for wipe_index in offset..=end {
						used_attacks[wipe_index as usize] = 0
					}

					found = false;
					break;
				}
			}

			if found {
				println!("    {},", magic.nr);
			}
		}

		offset += permutations;
	}
}

pub fn init_magic_numbers() {
	println!("pub const LEVEL_MAGIC_NUMBERS = [");
	generate_64_magic_numbers(LEVEL_MASKS, LEVEL_SQUARES, 102_400);
	println!("];\n");

	println!("pub const DIAGONAL_MAGIC_NUMBERS = [");
	generate_64_magic_numbers(DIAGONAL_MASKS, DIAGONAL_SQUARES, 5_248);
	println!("];");
}

pub fn init_magic_attacks(
	piece_masks: [u64; 64],
	target_squares: [&[&[u8]]; 64],
	magic_numbers: [u64; 64],
) -> ([Magic; 64], Vec<Vec<u64>>) {
	let mut magics: [Magic; 64] = [Default::default(); 64];
	let mut table: Vec<Vec<u64>> = Vec::new();

	let mut offset = 0;

	for square_index in 0..64 {
		table.push(Vec::new());

		let mask = piece_masks[square_index];
		let squares = target_squares[square_index];

		let bits = mask.count_ones() as u8;
		let permutations = (1u32 << bits) as usize;
		let end = offset + permutations - 1;

		let mut blockers = vec![0u64; permutations];
		let mut attacks = vec![0u64; permutations];
		let used_attacks = &mut table[square_index];

		for i in 0..permutations {
			blockers[i] = index_to_uint64(i, bits, mask);
			attacks[i] = sliding_attack(squares, blockers[i]);
		}

		let mut magic: Magic = Default::default();

		magic.mask = mask;
		magic.shift = 64 - bits;
		magic.offset = offset as u64;
		magic.nr = magic_numbers[square_index];

		for next in 0..permutations {
			let index = magic.get_index(blockers[next]);

			if used_attacks[index] == 0 {
				let fail_low = index < offset as usize;
				let fail_high = index > end as usize;
				assert!(!fail_low && !fail_high, "Indexing error. Error in Magics.");

				used_attacks[index] = attacks[next];
			} else {
				panic!("Attack table index not empty. Error in Magics.");
			}
		}

		magics[square_index] = magic;
		offset += permutations;
	}

	(magics, table)
}
