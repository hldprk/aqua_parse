use super::*;

mod repeated;
mod until;
mod list;

pub use list::*;
pub use until::*;
pub use repeated::*;

use std::ops::Deref;
use std::rc::Rc;
use std::fmt::Display;

/// Tries parsing `P : Parse`, returning `Ok(Self)`, 
/// otherwise returning `None` and leaving `index` alone.
impl<P : Parse> Parse for Option<P> {

	fn parse<'string>(string: &'string str, index: &mut usize) -> Result<'string, Self> {
		
		let ref mut cloned_index = index.clone();
		let result_maybe = P::parse(string, cloned_index);
		
		match result_maybe {
			
			Ok(_) => Ok(Some(P::parse(string, index)?)),
			Err(..) => Ok(None)
			
		}
		
	}
	
}

impl<P : Parse> Parse for Box<P> {
	
	fn parse<'string>(string: &'string str, index: &mut usize) -> Result<'string, Self> {
		
		Ok(Box::new(P::parse(string, index)?))
		
	}
	
}

impl<P : Parse> Parse for Rc<P> {
	
	fn parse<'string>(string: &'string str, index: &mut usize) -> Result<'string, Self> {

		Ok(Rc::new(P::parse(string, index)?))

	}

}

impl<P : Parse + Clone> Parse for Vec<P> {
	
	fn parse<'string>(string: &'string str, index: &mut usize) -> Result<'string, Self> {

		let repeated_maybe = Option::<MoreThan<0, P>>::parse(string, index)?;

		match repeated_maybe {

			Some(repeated) => {

				let vector = repeated.into_iter().cloned().collect();

				Ok(vector)

			},

			None => {

				let vector = Vec::default();

				Ok(vector)

			}

		}
		
	}

}

impl Parse for () {

	fn parse<'string>(_: &'string str, _: &mut usize) -> Result<'string, Self> {
		
		Ok(())

	}

}