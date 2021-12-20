use std::fmt::Debug;
use std::ops::RangeBounds;

use crate::*;

/// Parses a `&'static str` passed generically to `Literal`.
#[derive(Clone, PartialEq, Eq, Hash, Default)]
pub struct Literal<const LITERAL: &'static str>;

impl<const LITERAL: &'static str> Debug for Literal<LITERAL> {

	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {

		write!(f, "Literal<\"{}\">", LITERAL)
		
	}

}

impl<const LITERAL: &'static str> std::fmt::Display for Literal<LITERAL> {

	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {

		write!(f, "{}", LITERAL)
		
	}

}

impl<const LITERAL: &'static str> Parse for Literal<LITERAL> {

	fn parse(position: &mut Position) -> Result<Self>
	where Self : Sized {

		let error = Error::unexpected::<Self>(position.clone());
		
		if position.clone().next().is_none() {
	
			return Err(Error::unexpected_end::<Self>(position.clone()))

		}

		else if position.remainder().starts_with(LITERAL) {

			let length = LITERAL.len();

			for _ in 0..length {

				if position.clone().next().is_none() {
	
					return Err(Error::unexpected_end::<Self>(position.clone()))
	
				}
	
				else { let _ = position.next(); }
	
			}

			return Ok(Self);

		} 
		
		else { Err(error) }
		
	}

}