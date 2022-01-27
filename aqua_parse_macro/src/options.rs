use std::ops::BitAnd;
use std::ops::BitOr;
use syn::parse2;

use super::*;

/// Options passed to a type through their definitions' attributes.
#[derive(Debug, Clone)]
pub struct Options {

	pub(crate) is_strict: bool,
	pub(crate) error_types: Vec<Type>,
	pub(crate) description_maybe: Option<TokenStream>,
	pub(crate) label_maybe: Option<Literal>,
	pub(crate) pattern_maybe: Option<Literal>,

}

impl From<Vec<Attribute>> for Options {

	fn from(attributes: Vec<Attribute>) -> Self {
			
		// checks for an outer attribute named "strict"
		let is_strict = 
		attributes
		.iter()
		.find(|attribute| attribute.path.is_ident("strict"))
		.is_some();
		
		// extracts arguments from a `#[description(...)]` attribute if available
		let description_maybe = attributes
		.iter()
		.find_map(|attribute| {
			
			if attribute.path.is_ident("description") {
				
				let meta_maybe = attribute.parse_meta();

				let meta_list_maybe = match meta_maybe {

					Ok(meta) => match meta {

						Meta::List(meta_list) => Some(meta_list),
						_ => None

					}, 

					Err(_) => None

				};

				match meta_list_maybe {

					Some(meta_list) => Some(meta_list.nested.to_token_stream()),
					None => None

				}

			}

			else { None }

		});

		// extracts arguments from a `#[label(...)]` attribute if available
		let label_maybe = attributes
		.iter()
		.find_map(|attribute| {
			
			if attribute.path.is_ident("label") {

				let literal_maybe = attribute.parse_args::<Literal>();

				match literal_maybe {

					Ok(literal) => Some(literal),
					Err(_) => None

				}

			}

			else { None }

		});

		// extracts arguments from a `#[pattern(...)]` attribute if available
		let pattern_maybe = attributes
		.iter()
		.find_map(|attribute| {
			
			if attribute.path.is_ident("pattern") {

				let literal_maybe = attribute.parse_args::<Literal>();

				match literal_maybe {

					Ok(literal) => Some(literal),
					Err(_) => None

				}

			}

			else { None }

		});

		let error_types = attributes
		.iter()
		.filter_map(|attribute| {
			
			if attribute.path.is_ident("error") {

				let type_maybe = attribute.parse_args::<Type>();

				match type_maybe {

					Ok(t) => Some(t),
					Err(_) => None

				}

			}

			else { None }

		}).collect();

		Self { 
		
			is_strict,
			error_types, 
			label_maybe,
			description_maybe,
			pattern_maybe, 
		
		}
	
	}

}
