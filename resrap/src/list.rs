use std::borrow::Borrow;
use std::borrow::BorrowMut;

use crate::*;

/// Parses a comma-separated list of `P`, where `P : Parse`, with optional trailing comma.
///
/// `List<_>` always acts as though it is `#[padded]`.
#[derive(Debug)]
pub struct List<P : Parse>(pub Vec<P>);

impl<P: Parse> std::ops::Deref for List<P> {

    type Target = Vec<P>;

    fn deref(&self) -> &Self::Target { &self.0 }

}

impl<P : Parse> Parse for List<P> {

	fn parse(position: &mut Position) -> Result<Self> {

		let mut items = Vec::default();

		loop {

			let item_maybe = P::parse(position.clone().borrow_mut());

			if item_maybe.is_err() { break; }
			
			let _ = Option::<Vec::<Whitespace>>::parse(position)?;
			
			let item = P::parse(position)?;

			items.push(item);
			
			let comma_maybe = Token::<",">::parse(position.clone().borrow_mut());
			
			if comma_maybe.is_err() { break; }
			
			let _ = Option::<Vec::<Whitespace>>::parse(position)?;
			
			let _ = Token::<",">::parse(position)?;

			let _ = Option::<Vec::<Whitespace>>::parse(position)?;
			
		}

		Ok(List(items))

	}

}

impl<T> Clone for List<T> where T : Parse + Clone {

	fn clone(&self) -> Self {
		
		Self(self.0.clone())

	}

}

impl<T> PartialEq for List<T> where T : Parse + PartialEq {

	fn eq(&self, other: &Self) -> bool {
		
		self.0 == other.0

	}
	
}

impl<T> Eq for List<T> where T : Parse + Eq {}

impl<P> IntoIterator for List<P> where P : Parse {

	type Item = P;
	type IntoIter = std::vec::IntoIter<P>;

	fn into_iter(self) -> Self::IntoIter {
		
		self.0.into_iter()

	}

}