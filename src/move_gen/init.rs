use super::MoveGenerator;

use crate::{
	bitboard::{print_bitboard, set_bit, Bitboard},
	board::Board,
	color::Color,
	magic::{Magic, BISHOP_MAGIC_NR, ROOK_MAGIC_NR},
	move_gen::attack::AttackBoards,
	piece::Piece,
};

fn set_pawn_capture(
	bitboard: &mut Bitboard,
	square_index: i8,
	right_capture: i8,
	left_capture: i8,
	file: u8,
) {
	//* Right capture */
	if file != 0 {
		set_bit(&mut *bitboard, (square_index + right_capture) as u8);
	}

	//* Left capture */
	if file != 7 {
		set_bit(&mut *bitboard, (square_index + left_capture) as u8);
	}
}

impl MoveGenerator {
	pub fn init(&mut self) {
		self.init_king();
		self.init_pawn();
		self.init_knight();
		self.init_magic(Piece::Rook);
		self.init_magic(Piece::Bishop);

		for square_index in 0..64usize {
			self.square_bit[square_index] = 1 << square_index
		}
	}

	fn init_king(&mut self) {
		let king = &mut self.king[..];

		let level_offets = [1, 8, -1, -8];
		let diagonal_offsets = [7, 9, -7, -9];

		for square_index in 0..64 {
			let next = square_index as usize;

			let level_num_to_edge = MoveGenerator::rook_num_to_edge(square_index);
			let diagonal_num_to_edge = MoveGenerator::bishop_num_to_edge(square_index);

			for direction_index in 0..4 {
				//* Level */
				if level_num_to_edge[direction_index] > 0 {
					let target_square = square_index as i8 + level_offets[direction_index];
					set_bit(&mut king[next], target_square as u8);
				}

				//* Diagonal */
				if diagonal_num_to_edge[direction_index] > 0 {
					let target_square = square_index as i8 + diagonal_offsets[direction_index];
					set_bit(&mut king[next], target_square as u8);
				}
			}
		}
	}

	fn init_pawn(&mut self) {
		let pawn = &mut self.pawn[..];

		let white_index = Color::White.to_index();
		let black_index = Color::Black.to_index();

		for rank in 0..8 {
			for file in 0..8 {
				let square_index = Board::to_square_index(rank, file) as usize;

				//* White captures */
				if square_index < 56 {
					set_pawn_capture(
						&mut pawn[white_index][square_index],
						square_index as i8,
						7,
						9,
						file,
					);
				}

				//* Black captures */
				if square_index >= 8 {
					set_pawn_capture(
						&mut pawn[black_index][square_index],
						square_index as i8,
						-9,
						-7,
						file,
					);
				}
			}
		}
	}

	fn init_knight(&mut self) {
		let knight = &mut self.knight[..];

		let offsets = [17, 15, 10, 6, -6, -10, -15, -17];
		let range = 0..64;

		for rank in 0..8i8 {
			for file in 0..8i8 {
				let square_index = Board::to_square_index(rank as u8, file as u8) as i8;

				for offset in offsets {
					let target_square = square_index + offset;

					if range.contains(&target_square) {
						let y = target_square / 8;
						let x = target_square - y * 8;

						let max_distance = i8::max((file - x).abs(), (rank - y).abs());

						if max_distance == 2 {
							set_bit(&mut knight[square_index as usize], target_square as u8);
						}
					}
				}
			}
		}
	}

	fn init_magic(&mut self, piece: Piece) {
		let ok = piece == Piece::Rook || piece == Piece::Bishop;
		assert!(ok, "Illigal piece: {}", piece.to_full_name());

		let is_rook = piece == Piece::Rook;
		let mut offset = 0;

		let magics_table = if is_rook {
			&mut self.rook_magics[..]
		} else {
			&mut self.bishop_magics[..]
		};

		for square_index in 0..64 {
			let mask = if is_rook {
				MoveGenerator::rook_mask(square_index)
			} else {
				MoveGenerator::bishop_mask(square_index)
			};

			let bits = mask.count_ones();
			let permutations = 2u64.pow(bits);
			let end = offset + permutations - 1;

			let blocker_boards = MoveGenerator::blocker_boards(mask);
			let attack_boards = if is_rook {
				MoveGenerator::rook_attack_boards(square_index, &blocker_boards)
			} else {
				MoveGenerator::bishop_attack_boards(square_index, &blocker_boards)
			};

			let mut magic = Magic::default();

			magic.mask = mask;
			magic.shift = (64 - bits) as u8;
			magic.offset = offset;
			magic.nr = if is_rook {
				ROOK_MAGIC_NR[square_index as usize]
			} else {
				BISHOP_MAGIC_NR[square_index as usize]
			};

			for next in 0..permutations as usize {
				let index = magic.get_index(blocker_boards[next]);
				let attacks_table = if is_rook {
					&mut self.rook[..]
				} else {
					&mut self.bishop[..]
				};

				if attacks_table[index] == 0 {
					let fail_low = index < offset as usize;
					let fail_high = index > end as usize;
					assert!(!fail_low && !fail_high, "Indexing error. Error in Magics.");

					attacks_table[index] = attack_boards[next];
				} else {
					panic!("Attack table index not empty. Error in Magics.");
				}
			}

			magics_table[square_index as usize] = magic;
			offset += permutations;
		}
	}
}
