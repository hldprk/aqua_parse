use std::marker::PhantomData;
use super::*;
use std::fmt::Debug;

/// Given `A, B : Parse`, parses `A` until `B` can be parsed.
#[derive(Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Until<A : Parse, B : Parse> {

	elements: Vec<A>,
	span: Span,
	phantom_data: PhantomData<B>

}

impl<A : Parse, B : Parse> Until<A, B> {
	
	pub fn elements(&self) -> &[A] {

		self.elements.deref()

	}

}

impl<A : Parse, B : Parse> Deref for Until<A, B> {

	type Target = [A];

	fn deref(&self) -> &Self::Target {
		
		self.elements.deref()

	}

}

impl<A : Parse + Display, B : Parse> Display for Until<A, B> {

	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		
		for a in self.elements() {

			write!(f, "{a}")?;

		}

		Ok(())

	}

}

impl<A : Parse, B : Parse> Debug for Until<A, B> {

	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		
		write!(f, "{:?}", self.elements())

	}

} 

impl<A : Parse, B : Parse> Parse for Until<A, B> {
	
	fn span(&self) -> &Span {
		
		&self.span

	}
	
	fn parse(state: &mut State) -> Result<Self> {
		
		let mut elements = Vec::default();
		let page = state.page();
		let start_index = state.index();
		let mut end_index = state.index();

		loop {

			if B::can_parse(state) { break; }
			
			else {

				let ok = A::parse(state)?;

				end_index = state.index();

				elements.push(ok);

			} 

		}

		let phantom_data = Default::default();
		let range = start_index .. end_index;
		let span = Span::new(page, range);
		
		Ok(Self { phantom_data, elements, span })

	}

}
