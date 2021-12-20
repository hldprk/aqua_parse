use std::any::type_name;
use std::fmt::Debug;
use super::*;

/// Tells `Parse::find` which direction to traverse.
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Direction {

	Forward,
	Backward

}

/// Constructs some type by reading a `&str`.
pub trait Parse : Sized + Debug {

	/// Used in error messages to label some type being parsed.
	///
	/// Uses `type_name::<Self>` by default.
	fn label() -> String {

		type_name::<Self>().into()

	}

	/// Consumes a `&str` to yield `Result<Self>`.
	fn parse<'string>(string: &'string str, index: &mut usize) -> Result<'string, Self>;
	
	/// Returns whether `Self` will parse successfully.
	fn can_parse<'string>(string: &'string str, index: &mut usize) -> bool {

		let ref mut cloned_index = index.clone();

		Self::parse(string, cloned_index).is_ok()

	}

	/// Finds the first instance of `Self` in a `&str`, starting at `index`.
	fn find<'string>(string: &'string str, index: &mut usize, direction: Direction) -> Option::<Self> {
		
		loop {
			
			let end_index = match direction {

				Direction::Forward => string.len(),
				Direction::Backward => 0,

			};

			if index.clone() == end_index { 
				
				return None;
				
			}
			
			else if Self::can_parse(string, index) {
				
				let ok = Self::parse(string, index).unwrap();
				
				return Some(ok);
				
			} else {

				match direction {

					Direction::Forward => *index += 1,
					Direction::Backward => *index -= 1

				}
				
			}
			
		}
		
	}

	/// Whether an instance of `Self` is found in a `&str`.
	fn is_found_in<'string>(string: &'string str) -> bool {

		let ref mut index = 0;

		Self::find(string, index, Direction::Forward).is_some()

	}
	
	/// Finds all non-overlapping instances of `Self` in some `&str`.
	fn find_all<'string>(string: &'string str, index: &mut usize) -> Vec::<Self> {
		
		let mut parsed_values = Vec::default();
		
		loop {

			let ref mut cloned_index = index.clone();

			let start_index = cloned_index.clone();
			
			let found_maybe = Self::find(string, cloned_index, Direction::Forward);

			match found_maybe {

				Some(some) => parsed_values.push(some),
				None => break

			}

			let end_index = cloned_index.clone();

			let progress = match end_index - start_index == 0 {

				true => 1,
				false => end_index - start_index

			};

			*index += progress;

		}

		parsed_values

	}


}
