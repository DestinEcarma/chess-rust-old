#![allow(dead_code, unused_variables, unused_mut, unused_imports)]

use bitboard::{print_bitboard, set_bit};
use board::print_board_indices;
use color::Color;
use move_gen::MoveGen;

pub mod bitboard;
pub mod board;
pub mod color;
pub mod error;
pub mod magic;
pub mod move_gen;
pub mod piece;

fn main() {
	let mut move_gen = MoveGen::new();

}
