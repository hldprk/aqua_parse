use super::*;
use std::hash::*;

/// Comma-separated list, where trailing comma is optional.
#[derive(Debug, Clone, PartialEq, PartialOrd, Eq)]
pub struct List<P : Parse>{ 

	elements: Vec<P>,
	span: Span

}

impl<P : Parse> Deref for List<P> {

	type Target = [P];

	fn deref(&self) -> &Self::Target {
		
		self.elements.deref()

	}
	
}

impl<P: Hash + Parse> Hash for List<P> {

	fn hash<H: Hasher>(&self, state: &mut H) {
       
		Hash::hash(&**self, state)
    
	}

}


impl<P : Parse> Parse for List<P> {

	fn span(&self) -> &Span {
		
		&self.span

	}

	fn parse(state: &mut State) -> Result<Self> {

		#[pattern("[,]")]
		#[derive(Clone, Parse, PartialEq, Eq, PartialOrd, Ord, Hash)]
		pub struct Comma(Span);
		
		let mut elements = Vec::default();
		let start_index = state.index();
		
		loop {

			if P::can_parse(state) {

				let ok = P::parse(state)?;

				let _ = Many::<whitespace::Whitespace>::parse(state);

				elements.push(ok);

				if Comma::can_parse(state) {

					let _ = Comma::parse(state)?;

					let _ = Many::<whitespace::Whitespace>::parse(state);

				}

				else { break; }

			}

			else { break; }
	
		}

		let end_index = state.index();
		let range = start_index .. end_index;
		let page = state.page();

		let span = Span::new(page, range);	

		Ok( Self { elements, span } )

	}

}