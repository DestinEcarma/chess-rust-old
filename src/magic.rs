use rand::Rng;

use crate::bitboard::{pop_lsb, Bitboard};

#[derive(Default, Clone, Copy)]
pub struct Magic {
	pub mask: Bitboard,
	pub shift: u8,
	pub offset: u64,
	pub nr: u64,
}

impl Magic {
	pub fn get_index(&self, blocker: Bitboard) -> usize {
		(((blocker & self.mask).wrapping_mul(self.nr) >> self.shift) + self.offset) as usize
	}
}

fn random_u64_fewbits() -> u64 {
	let mut rng = rand::thread_rng();

	rng.gen::<u64>() & rng.gen::<u64>() & rng.gen::<u64>()
}

fn index_to_uint64(index: usize, bits: u8, mut bitboard: Bitboard) -> Bitboard {
	let mut result: Bitboard = 0;

	for i in 0..bits {
		let square_index = pop_lsb(&mut bitboard);

		if index & (1 << i) != 0 {
			result |= 1 << square_index;
		}
	}

	result
}
