use std::{str::FromStr, time::Instant};

use crate::{board::Board, move_gen::MoveGenerator};

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
	#[allow(dead_code)]
	pub fn set_fen(&mut self, fen: &str) {
		self.board = Board::from_str(fen).unwrap();
	}

	pub fn perft(&mut self, depth: usize) {
		let now = Instant::now();

		let mut nodes = 0;

		let move_list = self.move_gen.all_possible(&self.board);

		for _move in move_list {
			self.board.make_move(_move);
			let move_nodes = self._perft(depth - 1);
			self.board.undo_move();

			nodes += move_nodes;
			// println!("{_move}: {move_nodes}");
		}

		let elapsed = now.elapsed().as_millis() as f64;
		let nodes_per_seconds = ((nodes * 1000) as f64 / elapsed).floor();

		println!("\nTotal time (ms)\t: {:.0}", elapsed);
		println!("Nodes searched\t: {nodes}");
		println!("Nodes/second\t: {}", nodes_per_seconds);
	}

	fn _perft(&mut self, depth: usize) -> usize {
		if depth == 0 {
			return 1;
		}

		let mut nodes = 0;

		let move_list = self.move_gen.all_possible(&self.board);

		for _move in move_list {
			self.board.make_move(_move);
			let move_nodes = self._perft(depth - 1);
			self.board.undo_move();

			nodes += move_nodes;
		}

		return nodes;
	}
}
