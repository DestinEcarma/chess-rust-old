#![allow(
	dead_code,
	unused_mut,
	unused_imports,
	unused_variables,
	unused_assignments
)]

mod bitboard;
mod board;
mod color;
mod error;
mod magic;
mod move_gen;
mod notation;
mod piece;

use std::{str::FromStr, time::Instant};

use bitboard::{pop_lsb_to_bitboard, print_bitboard, print_board_indices, set_bit};
use board::Board;
use magic::find_magic;
use move_gen::MoveGenerator;
use piece::{Piece, ALL_PIECES};
use rand::Rng;

use crate::bitboard::pop_lsb;

fn random_u64_fewbits() -> u64 {
	let mut rng = rand::thread_rng();

	rng.gen::<u64>() & rng.gen::<u64>()
}

fn main() {
	print_board_indices();

	let mut move_gen = MoveGenerator::default();
	let board = match Board::from_str(&"8/8/8/7b/8/3q4/3P4/k2K3Q w - - 0 1") {
		Ok(board) => board,
		Err(e) => panic!("{e}"),
	};

	let mut move_list = Vec::new();
	let now = Instant::now();

	move_gen.calculate_attack_mask(&board);

	for piece in ALL_PIECES {
		match piece {
			Piece::Pawn => move_gen.pawns(&board, &mut move_list),
			_ => move_gen.piece(&board, piece, &mut move_list),
		}
	}

	let duration = now.elapsed();
	dbg!(&duration);
}
