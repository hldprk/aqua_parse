#![allow(unused_imports)]

use syn::*;
use quote::*;

mod struct_helper;
mod enum_helper;
mod derive_helper;

use struct_helper::*;
use enum_helper::*;
use derive_helper::*;


extern crate proc_macro;

/// Types that derive `Parse` will automatically implement `Parse`, provided its members or variants do also. 
///
///## `struct`
/// A `Parse` `struct` will parse each of its members in sequence, failing entirely if one can't be parsed.
///
///## `enum`
/// A `Parse` `enum` will parse each of its variants in the defined order until one succeeds, which is then used as the result.
///
///## `#[padded]`
/// 
/// When this attribute is used, `Option<Vec<Whitespace>>` is parsed before
/// and after parsing each child of this `struct` or `enum`.
#[proc_macro_derive(Parse, attributes(padded))]
pub fn parse_derive(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
	
	crate::derive_helper(input.into()).into()
	
}
