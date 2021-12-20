use std::fmt::Display;
use std::ops::Deref;
use std::ops::Range;

use super::*;

/// A hint for how many elements a [`Repeated`] should parse.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Bounds {

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

}

/// Greedily parses `P : Parse`, succeeding or failing based on some [`Bounds`].
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Repeated<const BOUNDS: Bounds, P : Parse>(Vec<P>);

pub type Exactly<const N : usize, P> = Repeated<{Bounds::exactly(N)}, P>;
pub type MoreThan<const N : usize, P> = Repeated<{Bounds::more_than(N)}, P>;
pub type LessThan<const N : usize, P> = Repeated<{Bounds::less_than(N)}, P>;
pub type Between<const RANGE : Range<usize>, P> = Repeated<{Bounds::between(RANGE)}, P>;

impl<const BOUNDS: Bounds, P : Parse> Deref for Repeated<BOUNDS, P> {

	type Target = [P];

	fn deref(&self) -> &Self::Target {
		
		self.0.deref()

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

	fn parse<'string>(string: &'string str, index: &mut usize) -> Result<'string, Self> {
		
		let mut elements = Vec::default();
		let start_index = index.clone();
		let end_index: usize;
		let mut error = Error::from(string);

		loop {

			if index.clone() == string.len() { 
				
				end_index = index.clone();
				break; 
			
			}

			let ref mut cloned_index = index.clone();
						
			let result = P::parse(string, cloned_index);

			match result {

				Ok(_) => {
					
					let parsed = P::parse(string, index).unwrap();

					elements.push(parsed);

					index.clone_from(cloned_index);

				},

				Err(element_error) => { 
					
					end_index = index.clone();
					error += element_error;
					break;
				
				}

			}

		}
							
		let range = match BOUNDS {

			Bounds::Exactly(n) => n .. (n + 1),

			Bounds::LessThan(n) => 0..n,

			Bounds::MoreThan(n) => (n + 1) .. usize::MAX,

			Bounds::Between(r) => r

		};

		if range.contains(&elements.len()) { Ok(Self(elements)) }
		else {

			let range_descriptor = match BOUNDS {

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
			let mut message = Message::start(start_index, Level::Error, description);
			message.end(end_index);

			error += message;

			Err(error)

		}

	}

}


impl<const BOUNDS: Bounds, P : Parse> FromIterator<P> for Repeated<BOUNDS, P> {

	fn from_iter<T: IntoIterator<Item = P>>(iterator: T) -> Self {
		
		let vector = Vec::from_iter(iterator);

		Self(vector)

	}

}