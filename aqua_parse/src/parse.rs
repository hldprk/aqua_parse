use std::any::type_name;
use std::fmt::Debug;
use super::*;

/// An interface for types able to be constructed from a [`State`].
pub trait Parse : Sized + Debug {
	
	fn span(&self) -> &Span;

	fn start(&self) -> Place {

		self.span().start()

	}

	fn end(&self) -> Place {

		self.span().end()

	}

	/// Used in error messages to label some type being parsed.
	fn label() -> String {

		type_name::<Self>().to_string()

	}

	/// `String` describing an `Error` resulting from a bad parse of `Self`.
	fn description() -> String {

		let label = Self::label();

		format!("Expected '{label}' here.")

	}

	/// Constructs an `Error` for if `Self::parse` fails.
	fn error(span: Span, level: Level) -> Error {

		let description = Self::description();

		Error::new(Some(span), level, &description)
		
	}

	/// Whether `parse` should succeed or not.
	fn is_error() -> bool { false }

	/// Consumes a `&str` to yield `Result<Self>`.
	fn parse(state: &mut State) -> Result<Self>;

	/// Returns whether `Self` will parse successfully.
	fn can_parse(state: &mut State) -> bool {

		let ref mut cloned_state = state.clone();

		Self::parse(cloned_state).is_ok()

	}

	/// Finds the first instance of `Self` in a `&str`, starting at `index`.
	fn find(state: &mut State) -> Option::<Self> {
		
		loop {
			
			let end_index = state.page().len();

			if state.index() == end_index { 
				
				return None;
				
			}
			
			else if Self::can_parse(state) {
				
				let ok = Self::parse(state).unwrap();
				
				return Some(ok);
				
			} 
			
			else { state.index += 1; }
			
		}
		
	}

	/// Whether an instance of `Self` is found in a `&str`.
	fn is_found_in(state: &mut State) -> bool {

		Self::find(state).is_some()

	}
	
	/// Finds all non-overlapping instances of `Self` in some `&str`.
	fn find_all(state: &mut State) -> Vec::<Self> {
		
		let mut parsed_values = Vec::default();
		
		let ref mut cloned_state = state.clone();

		loop {

			let start_index = state.index();
			
			let found_maybe = Self::find(cloned_state);

			match found_maybe {

				Some(some) => parsed_values.push(some),
				None => break

			}

			let end_index = state.index();

			let progress = match end_index - start_index == 0 {

				true => 1,
				false => end_index - start_index

			};

			state.index += progress

		}

		parsed_values

	}

}
