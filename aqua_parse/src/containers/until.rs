use std::marker::PhantomData;
use super::*;
use std::fmt::Debug;

/// Given `A, B : Parse`, parses `A` until `B` can be parsed.
#[derive(Default, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Until<A : Parse, B : Parse> {

	content: Vec<A>,
	phantom_data: PhantomData<B>

}

impl<A : Parse, B : Parse> Until<A, B> {
	
	pub fn content(&self) -> &[A] {

		self.content.deref()

	}

}

impl<A : Parse, B : Parse> Deref for Until<A, B> {

	type Target = [A];

	fn deref(&self) -> &Self::Target {
		
		self.content.deref()

	}

}

impl<A : Parse + Display, B : Parse> Display for Until<A, B> {

	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		
		for a in self.content() {

			write!(f, "{a}")?;

		}

		Ok(())

	}

}

impl<A : Parse, B : Parse> Debug for Until<A, B> {

	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		
		write!(f, "{:?}", self.content())

	}

} 

impl<A : Parse, B : Parse> Parse for Until<A, B> {

	fn parse<'string>(string: &'string str, index: &mut usize) -> Result<'string, Self> {
		
		let mut content = Vec::default();

		loop {

			if B::can_parse(string, index) { break; }
			
			else {

				let ok = A::parse(string, index)?;

				content.push(ok);

			} 

		}

		let phantom_data = Default::default();
		
		Ok(Self { phantom_data, content })

	}

} 