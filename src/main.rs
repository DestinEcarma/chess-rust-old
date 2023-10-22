// use std::io::{stdout, Write, stdin, Read};

use benchmark::Benchmark;

mod benchmark;
mod bitboard;
mod board;
mod color;
mod error;
mod magic;
mod move_gen;
mod notation;
mod piece;

fn main() {
	let mut bench = Benchmark::default();
	// bench.set_fen("rnb1kbnr/pp1ppppp/2p5/q7/P7/3P4/1PP1PPPP/RNBQKBNR w KQkq - 1 3");

	bench.perft(6);

	// let mut stdout = stdout();
	// stdout.write(b"Press Enter to continue...").unwrap();
	// stdout.flush().unwrap();
	// stdin().read(&mut [0]).unwrap();
}
