use std::borrow::Borrow;
use std::borrow::BorrowMut;
use std::fmt::Debug;

use crate::*;

/// Constructs the implementing type by fully or partially consuming a [Position].
pub trait Parse : Debug + Sized {

	/// Advances [Position], returning `Self` or [Error].
	fn parse(position: &mut Position) -> Result<Self>;

}


impl<P : Parse> Parse for Option<P> {

	fn parse(position: &mut Position) -> Result<Self> {
		
		let result_maybe = P::parse(position);

		match result_maybe {

			Ok(result) => Ok(Some(result)),
			Err(..) => Ok(None)

		}

	}

}

impl<P : Parse> Parse for Vec<P> {

	fn parse(position: &mut Position) -> Result<Self> {
		
		let mut parsed = Vec::default();
		let last_error;

		loop {

			let result = P::parse(position);

			if result.is_ok() { parsed.push(result.unwrap()); } 
			
			else { 
			
				last_error = Some(result.unwrap_err());
				
				break;
			
			}

		}

		if parsed.len() > 0 { Ok(parsed) } 
		
		else { Err(last_error.unwrap()) }

	}

}
