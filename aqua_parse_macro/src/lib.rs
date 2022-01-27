#![allow(unused_imports)]

use proc_macro2::*;
use syn::*;
use quote::*;
use syn::punctuated::Punctuated;
use syn::spanned::Spanned;
use syn::token::Comma;

mod enum_helper;
mod derive_helper;
mod options;
mod parameters;
mod struct_helper;
mod token_helper;

use parameters::*;
use struct_helper::*;
use enum_helper::*;
use derive_helper::*;
use options::*;
use token_helper::*;

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
/// ### `#[strict]` 
/// Disables `Space` being parsed before and after each `Parse` type.
/// 
/// ### `#[error(...)]`
/// Is used on a `struct` or variant and will format an error message for when `Error`
/// is returned from `Self::parse`.
/// 

#[proc_macro_derive(Parse, attributes(strict, pattern, error, label, errors))]
pub fn parse_derive(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
	
	crate::derive_helper(input.into()).into()
	
}
