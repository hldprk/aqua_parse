use std::borrow::Borrow;
use std::borrow::BorrowMut;
use std::fmt::Debug;
use std::rc::Rc;

use crate::*;

/// Constructs the implementing type by fully or partially consuming a [Position].
pub trait Parse : Debug + Sized {

	/// Advances [Position], returning `Self` or [Error].
	fn parse(position: &mut Position) -> Result<Self>;

}


impl<P : Parse> Parse for Option<P> {

	fn parse(position: &mut Position) -> Result<Self> {
		
		let result_maybe = P::parse(position.clone().borrow_mut());

		match result_maybe {

			Ok(_) => Ok(Some(P::parse(position)?)),
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

impl<P : Parse> Parse for Box<P> {

	fn parse(position: &mut Position) -> Result<Self> {

		Ok(Box::new(P::parse(position)?))

	}

}

impl<P : Parse> Parse for Rc<P> {

	fn parse(position: &mut Position) -> Result<Self> {

		Ok(Rc::new(P::parse(position)?))

	}

}

impl Parse for bool {

	fn parse(position: &mut Position) -> Result<Self> {
		
		let result = Pattern::<"(true|false)">::parse(position);

		match result {

			Ok(pattern) => {

				let boolean = pattern.matched_string().parse::<bool>().unwrap();

				Ok(boolean)

			},

			Err(error) => Err(error)

		}

	}

}