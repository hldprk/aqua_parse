use std::ops::BitAnd;
use std::ops::BitOr;
use syn::parse2;

use super::*;

/// Options passed to a type through their definitions' attributes.
#[derive(Debug, Clone)]
pub struct Options {

	pub(crate) is_strict: bool,
	pub(crate) pattern_maybe: Option<Literal>,
	pub(crate) literal_maybe: Option<Literal>, 

}

impl From<Vec<Attribute>> for Options {

	fn from(attributes: Vec<Attribute>) -> Self {
			
		// checks for an outer attribute named "strict"
		let is_strict = 
		attributes
		.iter()
		.find(|attribute| attribute.path.is_ident("strict"))
		.is_some();
		
		// extracts a `Literal` token from a 'pattern' attribute, if possible
		let pattern_maybe = attributes
		.iter()
		.find_map(|attribute| {
			
			if attribute.path.is_ident("pattern") {
				
				let tokens = attribute.parse_args::<Literal>();

				match tokens {

					Ok(literal) => Some(literal),
					Err(_) => None

				}

			}

			else { None }

		});

		// extracts a `Literal` token from a 'literal' attribute, if possible
		let literal_maybe = attributes
		.iter()
		.find_map(|attribute| {

			if attribute.path.is_ident("literal") {
				
				let tokens = attribute.parse_args::<Literal>();

				match tokens {

					Ok(literal) => Some(literal),
					Err(_) => None

				}

			}

			else { None }

		});

		Self { is_strict, pattern_maybe, literal_maybe }
	
	}

}
