#![allow(dead_code, unused_variables, unused_mut, unused_imports)]

use bitboard::{print_bitboard, print_board_indices, set_bit};
use board::Board;
use color::Color;
use move_gen::MoveGen;
use notation::Notation;

pub mod bitboard;
pub mod board;
pub mod color;
pub mod error;
pub mod magic;
pub mod move_gen;
pub mod notation;
pub mod piece;

fn main() {
	print_board_indices();

	print_bitboard(MoveGen::rook_mask(28), None);
}
