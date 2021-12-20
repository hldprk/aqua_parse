use std::ops::Deref;
use std::path::PathBuf;
use std::any::*;

use crate::*;

/// An `Error` type created by an unsuccessful parse.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Error {

	UnexpectedEnd { position: Position, expected: String },
	Branch { position: Position, expected: String, cause: Box<Error> },
	Unexpected { position: Position, expected: String }

}

impl Error {

	pub fn position(&self) -> &Position {

		match self {

			Error::UnexpectedEnd { position,.. } => position,
			Error::Unexpected { position,.. } => position,
			Error::Branch { position,.. } => position,

		}

	}

	pub fn unexpected_end<T : Any>(position: Position) -> Error {

		let expected = type_name::<T>().to_string();
		
		Error::UnexpectedEnd { position, expected }

	}

	pub fn branch<T : Any>(position: Position, cause: Box<Error>) -> Error {

		let expected = type_name::<T>().to_string();
		
		Error::Branch { expected, position, cause }

	}
	
	pub fn unexpected<T : Any>(position: Position) -> Error {

		let expected = type_name::<T>().to_string();
		
		Error::Unexpected { expected, position }

	}

}

pub type Result<T> = std::result::Result<T, Error>;

impl PartialOrd for Error {

	fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
		
		Some(self.position().clone().cmp(other.position().clone()))

	}

}


impl Ord for Error {

	fn cmp(&self, other: &Self) -> std::cmp::Ordering {
		
		self.position().clone().cmp(other.position().clone())

	}

}
