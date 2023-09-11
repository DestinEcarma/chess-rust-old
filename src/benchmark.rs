use std::{str::FromStr, time::Instant};

use crate::{
	bitboard::print_bitboard, board::Board, color::Color, move_gen::MoveGenerator, piece::Piece,
};

pub struct Benchmark {
	pub board: Board,
	move_gen: MoveGenerator,
}

impl Default for Benchmark {
	fn default() -> Self {
		Self {
			board: Board::default(),
			move_gen: MoveGenerator::default(),
		}
	}
}

impl Benchmark {
	pub fn set_fen(&mut self, fen: &str) {
		self.board = Board::from_str(fen).unwrap();
	}

	pub fn perft(&mut self, depth: usize) {
		let now = Instant::now();

		let mut nodes = 0;

		let move_list = self.move_gen.all_possible(&self.board);

		for _move in move_list {
			// let mut to_print = false;

			// if format!("{_move}") == "e1d2" {
			// 	to_print = true
			// }

			self.board.make_move(_move);
			let move_nodes = self._perft(depth - 1);
			self.board.undo_move();

			nodes += move_nodes;
			println!("{_move}: {move_nodes}");
		}

		let mili_seconds = now.elapsed().as_millis() as f64;

		println!("\nTotal time (ms)\t: {:.0}", mili_seconds);
		println!("Nodes searched\t: {nodes}");
		println!(
			"Nodes/second\t: {}",
			f64::round(
				(nodes as f64)
					/ if mili_seconds == 0.0 {
						1.0
					} else {
						mili_seconds / 1000.0
					}
			)
		);
	}

	fn _perft(&mut self, depth: usize) -> usize {
		if depth == 0 {
			return 1;
		}

		let mut nodes = 0;

		let move_list = self.move_gen.all_possible(&self.board);

		// if to_print {
		// 	println!("{}", self.board);
		// }

		for _move in move_list {
			self.board.make_move(_move);
			let move_nodes = self._perft(depth - 1);
			self.board.undo_move();

			nodes += move_nodes;

			// if to_print {
			// 	println!("{_move}: {move_nodes}");
			// }
		}

		return nodes;
	}
}
