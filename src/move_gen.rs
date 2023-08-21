use crate::{
	bitboard::{self, print_bitboard, set_bit, Bitboard},
	color::Color,
	magic::Magic,
};

pub struct MoveGen {
	king: [Bitboard; 64],
	pawn: [[Bitboard; 64]; 2],
	knight: [Bitboard; 64],
	rook: Vec<Bitboard>,
	bishop: Vec<Bitboard>,
	rook_magics: [Magic; 64],
	bishop_magics: [Magic; 64],
}

fn set_pawn_capture(
	bitboard: &mut Bitboard,
	square_index: i8,
	right_capture: i8,
	left_capture: i8,
	file: usize,
) {
	//* Right capture */
	if file != 0 {
		set_bit(&mut *bitboard, (square_index + right_capture) as u8);
		*bitboard = *bitboard;
	}

	//* Left capture */
	if file != 7 {
		set_bit(&mut *bitboard, (square_index + left_capture) as u8);
	}
}

impl MoveGen {
	pub fn new() -> Self {
		let move_gen = MoveGen {
			king: [0; 64],
			pawn: [[0; 64]; 2],
			knight: [0; 64],
			rook: vec![0; 102_400],
			bishop: vec![0; 5_248],
			rook_magics: [Magic::default(); 64],
			bishop_magics: [Magic::default(); 64],
		};

		move_gen
	}

	pub fn init_pawn(&mut self) {
		let pawn = &mut self.pawn;

		let white_index = Color::White.to_index();
		let black_index = Color::Black.to_index();

		for rank in 0..8usize {
			for file in 0..8usize {
				let square_index = rank * 8 + file;

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

	pub fn init_knight(&mut self) {
		let knight = &mut self.knight;

		let offsets = [17, 15, 10, 6, -6, -10, -15, -17];

		for rank in 0..8i8 {
			for file in 0..8i8 {
				let square_index = rank * 8 + file;

				for offset in offsets {
					let target_square = square_index + offset;

					if target_square >= 0 && target_square < 64 {
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
}
