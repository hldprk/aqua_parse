use std::ops::Deref;
use std::path::PathBuf;
use ansi_term::*;

use crate::*;

/// An `Error` type created by an unsuccessful parse.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Error {

	pub identifier: String,
	pub position: Position,
	pub cause: Option<Box<Error>>,

}

pub type Result<T> = std::result::Result<T, Error>;

impl PartialOrd for Error {

	fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
		
		self.position.clone().partial_cmp(other.position.clone())

	}

}


impl Ord for Error {

	fn cmp(&self, other: &Self) -> std::cmp::Ordering {
		
		self.position.clone().cmp(other.position.clone())

	}

}

impl std::fmt::Display for Error {

	fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		
		let mut current_error = Some(Box::new(self.clone()));

		let first_message = "resrap error:";
		
		let red_first_message = Color::Red.paint(first_message);

		writeln!(formatter, "{}", red_first_message)?;
		
		while current_error.is_some() {

			let cloned_unwraped = current_error.clone().unwrap();
			
			let message = format!(
				
				"    from '{}' at row {}, column {}", 
				cloned_unwraped.identifier,
				cloned_unwraped.position.row(),
				cloned_unwraped.position.column(),

			);
			
			let red_message = Color::Red.paint(message);
			
			writeln!(formatter, "{}", red_message)?;
			
			current_error = current_error.unwrap().cause;

		}

		Ok(())

	}

}