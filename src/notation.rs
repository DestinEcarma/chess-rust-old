use std::{
	fmt::{Display, Formatter, Result},
	str::FromStr,
};

use crate::error::Error;

#[rustfmt::skip]
#[derive(Copy, Clone)]
pub enum Notation {
	A8, B8, C8, D8, E8, F8, G8, H8,
	A7, B7, C7, D7, E7, F7, G7, H7,
	A6, B6, C6, D6, E6, F6, G6, H6,
	A5, B5, C5, D5, E5, F5, G5, H5,
	A4, B4, C4, D4, E4, F4, G4, H4,
	A3, B3, C3, D3, E3, F3, G3, H3,
	A2, B2, C2, D2, E2, F2, G2, H2,
	A1, B1, C1, D1, E1, F1, G1, H1,
}

impl From<usize> for Notation {
	fn from(value: usize) -> Self {
		match value {
			00 => Notation::A1,
			01 => Notation::B1,
			02 => Notation::C1,
			03 => Notation::D1,
			04 => Notation::E1,
			05 => Notation::F1,
			06 => Notation::G1,
			07 => Notation::H1,
			08 => Notation::A2,
			09 => Notation::B2,
			10 => Notation::C2,
			11 => Notation::D2,
			12 => Notation::E2,
			13 => Notation::F2,
			14 => Notation::G2,
			15 => Notation::H2,
			16 => Notation::A3,
			17 => Notation::B3,
			18 => Notation::C3,
			19 => Notation::D3,
			20 => Notation::E3,
			21 => Notation::F3,
			22 => Notation::G3,
			23 => Notation::H3,
			24 => Notation::A4,
			25 => Notation::B4,
			26 => Notation::C4,
			27 => Notation::D4,
			28 => Notation::E4,
			29 => Notation::F4,
			30 => Notation::G4,
			31 => Notation::H4,
			32 => Notation::A5,
			33 => Notation::B5,
			34 => Notation::C5,
			35 => Notation::D5,
			36 => Notation::E5,
			37 => Notation::F5,
			38 => Notation::G5,
			39 => Notation::H5,
			40 => Notation::A6,
			41 => Notation::B6,
			42 => Notation::C6,
			43 => Notation::D6,
			44 => Notation::E6,
			45 => Notation::F6,
			46 => Notation::G6,
			47 => Notation::H6,
			48 => Notation::A7,
			49 => Notation::B7,
			50 => Notation::C7,
			51 => Notation::D7,
			52 => Notation::E7,
			53 => Notation::F7,
			54 => Notation::G7,
			55 => Notation::H7,
			56 => Notation::A8,
			57 => Notation::B8,
			58 => Notation::C8,
			59 => Notation::D8,
			60 => Notation::E8,
			61 => Notation::F8,
			62 => Notation::G8,
			63 => Notation::H8,
			_ => panic!("Invalid number: {}", value),
		}
	}
}

impl FromStr for Notation {
	type Err = Error;

	fn from_str(value: &str) -> std::result::Result<Self, Self::Err> {
		match value {
			"a1" => Ok(Notation::A1),
			"a2" => Ok(Notation::A2),
			"a3" => Ok(Notation::A3),
			"a4" => Ok(Notation::A4),
			"a5" => Ok(Notation::A5),
			"a6" => Ok(Notation::A6),
			"a7" => Ok(Notation::A7),
			"a8" => Ok(Notation::A8),
			"b1" => Ok(Notation::B1),
			"b2" => Ok(Notation::B2),
			"b3" => Ok(Notation::B3),
			"b4" => Ok(Notation::B4),
			"b5" => Ok(Notation::B5),
			"b6" => Ok(Notation::B6),
			"b7" => Ok(Notation::B7),
			"b8" => Ok(Notation::B8),
			"c1" => Ok(Notation::C1),
			"c2" => Ok(Notation::C2),
			"c3" => Ok(Notation::C3),
			"c4" => Ok(Notation::C4),
			"c5" => Ok(Notation::C5),
			"c6" => Ok(Notation::C6),
			"c7" => Ok(Notation::C7),
			"c8" => Ok(Notation::C8),
			"d1" => Ok(Notation::D1),
			"d2" => Ok(Notation::D2),
			"d3" => Ok(Notation::D3),
			"d4" => Ok(Notation::D4),
			"d5" => Ok(Notation::D5),
			"d6" => Ok(Notation::D6),
			"d7" => Ok(Notation::D7),
			"d8" => Ok(Notation::D8),
			"e1" => Ok(Notation::E1),
			"e2" => Ok(Notation::E2),
			"e3" => Ok(Notation::E3),
			"e4" => Ok(Notation::E4),
			"e5" => Ok(Notation::E5),
			"e6" => Ok(Notation::E6),
			"e7" => Ok(Notation::E7),
			"e8" => Ok(Notation::E8),
			"f1" => Ok(Notation::F1),
			"f2" => Ok(Notation::F2),
			"f3" => Ok(Notation::F3),
			"f4" => Ok(Notation::F4),
			"f5" => Ok(Notation::F5),
			"f6" => Ok(Notation::F6),
			"f7" => Ok(Notation::F7),
			"f8" => Ok(Notation::F8),
			"g1" => Ok(Notation::G1),
			"g2" => Ok(Notation::G2),
			"g3" => Ok(Notation::G3),
			"g4" => Ok(Notation::G4),
			"g5" => Ok(Notation::G5),
			"g6" => Ok(Notation::G6),
			"g7" => Ok(Notation::G7),
			"g8" => Ok(Notation::G8),
			"h1" => Ok(Notation::H1),
			"h2" => Ok(Notation::H2),
			"h3" => Ok(Notation::H3),
			"h4" => Ok(Notation::H4),
			"h5" => Ok(Notation::H5),
			"h6" => Ok(Notation::H6),
			"h7" => Ok(Notation::H7),
			"h8" => Ok(Notation::H8),
			_ => Err(Error::InvalidStr {
				str: value.to_string(),
			}),
		}
	}
}

impl Display for Notation {
	fn fmt(&self, f: &mut Formatter) -> Result {
		write!(
			f,
			"{}",
			match self {
				Notation::A1 => "a1",
				Notation::A2 => "a2",
				Notation::A3 => "a3",
				Notation::A4 => "a4",
				Notation::A5 => "a5",
				Notation::A6 => "a6",
				Notation::A7 => "a7",
				Notation::A8 => "a8",
				Notation::B1 => "b1",
				Notation::B2 => "b2",
				Notation::B3 => "b3",
				Notation::B4 => "b4",
				Notation::B5 => "b5",
				Notation::B6 => "b6",
				Notation::B7 => "b7",
				Notation::B8 => "b8",
				Notation::C1 => "c1",
				Notation::C2 => "c2",
				Notation::C3 => "c3",
				Notation::C4 => "c4",
				Notation::C5 => "c5",
				Notation::C6 => "c6",
				Notation::C7 => "c7",
				Notation::C8 => "c8",
				Notation::D1 => "d1",
				Notation::D2 => "d2",
				Notation::D3 => "d3",
				Notation::D4 => "d4",
				Notation::D5 => "d5",
				Notation::D6 => "d6",
				Notation::D7 => "d7",
				Notation::D8 => "d8",
				Notation::E1 => "e1",
				Notation::E2 => "e2",
				Notation::E3 => "e3",
				Notation::E4 => "e4",
				Notation::E5 => "e5",
				Notation::E6 => "e6",
				Notation::E7 => "e7",
				Notation::E8 => "e8",
				Notation::F1 => "f1",
				Notation::F2 => "f2",
				Notation::F3 => "f3",
				Notation::F4 => "f4",
				Notation::F5 => "f5",
				Notation::F6 => "f6",
				Notation::F7 => "f7",
				Notation::F8 => "f8",
				Notation::G1 => "g1",
				Notation::G2 => "g2",
				Notation::G3 => "g3",
				Notation::G4 => "g4",
				Notation::G5 => "g5",
				Notation::G6 => "g6",
				Notation::G7 => "g7",
				Notation::G8 => "g8",
				Notation::H1 => "h1",
				Notation::H2 => "h2",
				Notation::H3 => "h3",
				Notation::H4 => "h4",
				Notation::H5 => "h5",
				Notation::H6 => "h6",
				Notation::H7 => "h7",
				Notation::H8 => "h8",
			}
		)
	}
}

impl Notation {
	// pub fn print_notation_board() {
	// 	let files = ["a", "b", "c", "d", "e", "f", "g", "h"];
	// 	let ranks = 1..=8;

	// 	let mut board = String::new();

	// 	for rank in 0..8 {
	// 		for file in 0..8 {
	// 			board += &format!(
	// 				"{:02} => Notation::{}{},\n",
	// 				Board::to_square_index(rank, file),
	// 				files[file as usize].to_uppercase(),
	// 				rank + 1,
	// 			);
	// 		}
	// 	}

	// 	println!("{board}\n")
	// }

	pub fn to_index(&self) -> usize {
		*self as usize
	}

	pub fn to_string(&self) -> String {
		format!("{}", self)
	}
}
