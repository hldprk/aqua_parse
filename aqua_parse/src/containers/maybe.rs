use super::*;

/// Like an `Option`, but for [`Parse`] types.
#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Maybe<P : Parse> {

	Some { 

		value: P,
		span: Span,

	},

	None(Span)

}

impl<P : Parse> Debug for Maybe<P> {

	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		
		match self {

			Self::Some { value, .. } => write!(f, "Some({value:?})"),
			Self::None(_) => write!(f, "None")

		}

	}

}

impl<P : Parse> Parse for Maybe<P> {

	fn span(&self) -> &Span {
		
		match self {
		
			Self::Some { span, .. } => {
				
				span

			},

			Self::None(span) => span
			
		}

	}

	fn parse(state: &mut State) -> Result<Self> {
		
		let ref mut cloned_state = state.clone();

		let start_index = state.index();

		let result = P::parse(cloned_state);

		let page = state.page();
		
		match result {

			Ok(ok) => {
				
				state.clone_from(cloned_state);

				let end_index = state.index();

				let range = start_index .. end_index;

				let span = Span::new(page, range);

				let value = ok;

				Ok(Self::Some { value, span })

			}, 

			Err(_) => {
				
				let range = start_index .. start_index;
				let span = Span::new(page, range);

				Ok(Self::None(span))

			}

		}

	}

}
