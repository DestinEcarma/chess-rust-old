#![allow(
	dead_code,
	unused_mut,
	unused_imports,
	unused_variables,
	unused_assignments
)]

mod benchmark;
mod bitboard;
mod board;
mod color;
mod error;
mod magic;
mod move_gen;
mod notation;
mod piece;

use std::{str::FromStr, time::Instant};

use benchmark::Benchmark;
use bitboard::{pop_lsb_to_bitboard, print_bitboard, print_board_indices, set_bit};
use board::Board;
use magic::find_magic;
use move_gen::MoveGenerator;
use piece::{Piece, ALL_PIECES};
use rand::Rng;

use crate::{bitboard::pop_lsb, color::Color};

fn main() {
	let mut bench = Benchmark::default();
	bench.set_fen("r2q1rk1/pP1p2pp/Q4n2/bbp1p3/Np6/1B3NBn/pPPP1PPP/R3K2R b KQ - 0 1 ");

	bench.perft(5);

	// const PIECE_MOVED: usize = 0;
	// const START_SQUARE: usize = 3;
	// const TARGET_SQUARE: usize = 9;
	// const MOVE_TYPE: usize = 15;
	// const PROMOTION: usize = 18;
	// const CASTLE_RIGHT: usize = 21;
	// const PREV_ENPASSANT: usize = 25;
	// const PREV_CASTLE_RIGHTS: usize = 31;
	// const PIECE_CAPTURED: usize = 35;
	
	// const PIECE_MOVED_MASK: u64 = 0x0000000007;
	// const START_SQUARE_MASK: u64 = 0x00000001f8;
	// const TARGET_SQUARE_MASK: u64 = 0x0000007e00;
	// const MOVE_TYPE_MASK: u64 = 0x0000038000;
	// const PROMOTION_MASK: u64 = 0x00001c0000;
	// const CASTLE_RIGHT_MASK: u64 = 0x0001e00000;
	// const PREV_ENPASSANT_MASK: u64 = 0x007e000000;
	// const PREV_CASTLE_RIGHTS_MASK: u64 = 0x0780000000;
	// const PIECE_CAPTURED_MASK: u64 = 0x3800000000;

	// println!("{:038b}: {:02}", PIECE_MOVED_MASK, PIECE_MOVED_MASK.trailing_zeros());
	// println!("{:038b}: {:02}", START_SQUARE_MASK, START_SQUARE_MASK.trailing_zeros());
	// println!("{:038b}: {:02}", TARGET_SQUARE_MASK, TARGET_SQUARE_MASK.trailing_zeros());
	// println!("{:038b}: {:02}", MOVE_TYPE_MASK, MOVE_TYPE_MASK.trailing_zeros());
	// println!("{:038b}: {:02}", PROMOTION_MASK, PROMOTION_MASK.trailing_zeros());
	// println!("{:038b}: {:02}", CASTLE_RIGHT_MASK, CASTLE_RIGHT_MASK.trailing_zeros());
	// println!("{:038b}: {:02}", PREV_ENPASSANT_MASK, PREV_ENPASSANT_MASK.trailing_zeros());
	// println!("{:038b}: {:02}", PREV_CASTLE_RIGHTS_MASK, PREV_CASTLE_RIGHTS_MASK.trailing_zeros());
	// println!("{:038b}: {:02}", PIECE_CAPTURED_MASK, PIECE_CAPTURED_MASK.trailing_zeros());
}
