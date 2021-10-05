#![allow(unused_imports)]

use syn::*;
use quote::*;

mod parse_helper;
mod padded_helper;

use parse_helper::*;
use padded_helper::*;

extern crate proc_macro;

/// Types that derive `Parse` will automatically implement `Parse`, provided its members or variants do also. 
///
/// A `Parse` `struct` will parse each of its members in sequence, failing entirely if one can't be parsed.
///
/// A `Parse` `enum` will parse each of its variants in the defined order until one succeeds, which is then used as the result.
#[proc_macro_derive(Parse)]
pub fn parse_derive(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
	
	crate::parse_derive_helper(input.into()).into()
	
}

/// Converts each member `T` of a `Parse` type into one typed `Padded<T>`.
#[proc_macro_attribute]
pub fn padded(attribute: proc_macro::TokenStream, item: proc_macro::TokenStream) -> proc_macro::TokenStream {

	crate::padded_helper(attribute.into(), item.into()).into()

}
