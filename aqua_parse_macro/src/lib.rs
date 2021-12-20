#![allow(unused_imports)]

use proc_macro2::*;
use syn::*;
use quote::*;
use syn::spanned::Spanned;

mod enum_helper;
mod derive_helper;
mod options;
mod struct_helpers; 

use struct_helpers::*;
use enum_helper::*;
use derive_helper::*;
use options::*;

extern crate proc_macro;

/// Types that derive `Parse` will automatically implement `Parse`, provided its members or fields do also.
/// 
/// An implementation of `Parse` for a `struct` will parse each of its fields in sequence, failing completely
/// if one returns `Err`.
/// 
/// An `enum` parses it variants in the defined order starting at the same index, choosing and initializing the first variant to parse
/// successfully.
/// 
/// ## Attributes
/// 
/// The `#[strict]` attribute disables `Space` being parsed before and after each `Parse` type.
/// 
/// The `#[pattern(...)]` attribute accepts a string literal as a regular expression, 
/// implementing `Parse` so that it matches that expression and stores it as a `String`.
/// 
/// At least for now, a `pattern` struct can only be a single-field tuple-struct containing a `String`.
/// 
/// ```
/// #[pattern(r"\w+")]
/// #[derive(Parse, Debug)]
/// pub struct Word(String);
///	assert!(Word::can_parse("asdf", &mut 0))
/// ```
/// 
/// 
/// The `#[literal(...)]` attribute also accepts a string literal, but just does a simple
/// string comparison with the remaining input, not storing it.
/// 
/// 
/// ```
/// #[literal("asdf")]
/// #[derive(Parse, Debug)]
/// pub struct Asdf;
///	assert!(Asdf::can_parse("asdf", &mut 0))
/// ```
/// ---
/// 
/// Also each `enum` variant allows the same attributes as a `struct`, because they derive `Parse` identically.
/// 
/// ```
/// #[derive(Parse, Debug)]
/// enum Operator {
/// 	
/// 	#[literal("*")]
/// 	Multiply,
/// 	#[literal("/")]
/// 	Divide,
/// 	#[literal("+")]
/// 	Plus,
/// 	#[literal("-")]
/// 	Minus,
/// 
/// }
/// 
/// assert!(Vec::<Operator>::can_parse(r"+-**//-", &mut 0))
/// 
/// ```
#[proc_macro_derive(Parse, attributes(strict, pattern, literal))]
pub fn parse_derive(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
	
	crate::derive_helper(input.into()).into()
	
}


#[cfg(test)]
mod tests {

	use crate::*;
	use crate::derive_helper::*;

	#[test]
	fn struct_test() {

		let tokens = 
		quote!(#[derive(Parse, Debug)]
		
			struct OneTwoThree {

				#[literal("1")]
				one: One,
				peek: Peek

			}
		
		);

		let output = derive_helper(tokens);

		println!("{output}")

	}

}