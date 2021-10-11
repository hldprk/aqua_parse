use std::ops::RangeBounds;

use crate::*;

/// Parses a `&'static str` passed generically to `Token`.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct Token<const TOKEN: &'static str>;

impl<const TOKEN: &'static str> Token<TOKEN> {

	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {

		write!(f, "{}", TOKEN)
		
	}

}

impl<const TOKEN: &'static str> Parse for Token<TOKEN> {

	fn parse(position: &mut Position) -> Result<Self>
	where Self : Sized {

		let start_position = position.clone();

		let error = Error {

			identifier: TOKEN.to_string(),
			position: start_position.clone(),
			cause: None

		};

		if position.index() >= position.source().len() { Err(error) }

		else if position.source()[position.index() .. position.index() + TOKEN.len()] == *TOKEN{

			let length = TOKEN.len();

			for _ in 0..length {

				let _ = position.next();

			}

			return Ok(Self);

		} 
		
		else { Err(error) }
		
	}

}
