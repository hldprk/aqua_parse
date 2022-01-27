use super::*;

mod repeated;
mod until;
mod list;
mod maybe;

pub use maybe::*;
pub use list::*;
pub use until::*;
pub use repeated::*;

use std::ops::Deref;
use std::fmt::Display;

impl<P : Parse> Parse for Box<P> {
	
	fn span(&self) -> &Span {
		
		self.as_ref().span()

	}

	fn parse(state: &mut State) -> Result<Self> {
		
		Ok(Box::new(P::parse(state)?))
		
	}
	
}


impl<P : Parse> Parse for Rc<P> {

	fn span(&self) -> &Span {
		
		self.as_ref().span()

	}

	fn parse(state: &mut State) -> Result<Self> {

		Ok(Rc::new(P::parse(state)?))

	}

}
