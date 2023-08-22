use failure::Fail;

#[derive(Clone, Debug, Fail)]
pub enum Error {
	#[fail(display = "Invalid FEN string: {}", fen)]
	InvalidFen { fen: String },

	#[fail(display = "Invalid str: {}", str)]
	InvalidStr { str: String },
}
