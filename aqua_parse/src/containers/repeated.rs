use std::fmt::Display;
use std::ops::Deref;
use std::ops::Range;

use super::*;

/// A hint for how many elements a [`Repeated`] should parse.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Bounds {

	/// Asserts that a `Repeated` parses 0 or more times.
	Many,
	/// Asserts that a `Repeated` parses an exact amount of elements.
	Exactly(usize),
	/// Asserts that a `Repeated` parses an more than `n` elements.
	MoreThan(usize),
	/// Asserts that a `Repeated` parses less than `n` elements.
	LessThan(usize),
	/// Asserts that a `Repeated` parses `n..m` elements.
	Between(Range<usize>),
	

}

impl Bounds {

	pub const fn exactly(n : usize) -> Bounds { Bounds::Exactly(n) }
	pub const fn more_than(n : usize) -> Bounds { Bounds::MoreThan(n) }
	pub const fn less_than(n : usize) -> Bounds { Bounds::LessThan(n) }
	pub const fn between(range: Range<usize>) -> Bounds { Bounds::Between(range) }
	pub const fn many() -> Bounds { Bounds::Many }

}

/// Greedily parses `P : Parse`, succeeding or failing based on some [`Bounds`].
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Repeated<const BOUNDS: Bounds, P : Parse>{

	elements: Vec<P>,
	span: Span

}

/// `Repeated<{Bounds::exactly(N)}, P>`
pub type Exactly<const N : usize, P> = Repeated<{Bounds::exactly(N)}, P>;

/// `Repeated<{Bounds::many()}, P>`
pub type Many<P> = Repeated<{Bounds::many()}, P>;

/// `Repeated<{Bounds::more_than(N)}, P>`
pub type MoreThan<const N : usize, P> = Repeated<{Bounds::more_than(N)}, P>;

/// `Repeated<{Bounds::less_than(N)}, P>`
pub type LessThan<const N : usize, P> = Repeated<{Bounds::less_than(N)}, P>;

/// `Repeated<{Bounds::between(RANGE)}, P>`
pub type Between<const RANGE : Range<usize>, P> = Repeated<{Bounds::between(RANGE)}, P>;

impl<const BOUNDS: Bounds, P : Parse> Deref for Repeated<BOUNDS, P> {

	type Target = [P];

	fn deref(&self) -> &Self::Target {
		
		self.elements.deref()

	}
	
}

impl<const BOUNDS: Bounds, P : Parse + Display> Display for Repeated<BOUNDS, P> {

	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		
		for p in self.iter() {

			write!(f, "{p}")?

		}

		Ok(())

	}

}

impl<const BOUNDS: Bounds, P : Parse> Parse for Repeated<BOUNDS, P> {
	
	fn span(&self) -> &Span {
		
		&self.span

	}
	
	fn parse(state: &mut State) -> Result<Self> {
		
		let mut elements = Vec::default();
		let start_index = state.index();

		loop {

			if state.index() == state.page().len() { break; }

			let ref mut cloned_state = state.clone();
						
			let result = P::parse(cloned_state);

			match result {

				Ok(_) => {
					
					let parsed = P::parse(state).unwrap();

					elements.push(parsed);

					state.clone_from(cloned_state);

				},

				Err(_) => { break; }

			}

		}
							
		let range = match BOUNDS {

			Bounds::Exactly(n) => n .. (n + 1),

			Bounds::LessThan(n) => 0..n,

			Bounds::MoreThan(n) => (n + 1) .. usize::MAX,

			Bounds::Between(r) => r,

			Bounds::Many => 0..usize::MAX

		};

		let end_index = state.index();

		let page = state.page();
		let span_range = start_index .. end_index;
		let span = Span::new(page, span_range);
		let span_maybe = Some(span.clone());

		if range.contains(&elements.len()) { 

			let result = Self { elements, span };

			Ok(result)
		
		}

		else {

			let range_descriptor = match BOUNDS {

				Bounds::Many => format!("any amount of"),
				
				Bounds::Exactly(n) => format!("exactly {n}"),
	
				Bounds::LessThan(n) => format!("less than {n}"),
	
				Bounds::MoreThan(n) => format!("more than {n}"),
	
				Bounds::Between(r) => {
					
					let n = r.start;
					let m = r.end;
					
					format!("({n}, {m}]")
				
				}

			};

			let label = P::label();
			let found_elements = elements.len();
			let description = &format!("Expected {range_descriptor} '{label}', but found {found_elements}.");
			let error = Error::new(span_maybe, Level::Error, &description);

			Err(error)

		}

	}

}
