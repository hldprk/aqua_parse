#![allow(dead_code)]
#![allow(legacy_derive_helpers)]
#![allow(const_evaluatable_unchecked)]
#![allow(incomplete_features)]
#![feature(adt_const_params)]
#![feature(generic_const_exprs)]						


//! A general-use parsing library made to use with the `aqua` crate.
use std::rc::*;
use std::fmt::Debug;
use aqua_error::*;
use regex::Regex;
use std::ops::Deref;

/// Some iterable parsers.
pub mod containers;

/// Definitions of whitespace parsers.
pub mod whitespace;

mod parse;
mod state;

pub use state::*;
pub use parse::*;

pub use aqua_parse_macro::*;

/// `Result<T, aqua_error::Error>`
pub type Result<T> = std::result::Result<T, Error>;

#[cfg(test)]
mod tests {

	use crate::*;
	use crate::containers::*;

	#[pattern("1")]
	#[derive(Clone, Parse, PartialEq, Eq, PartialOrd, Ord, Hash)]
	pub struct One(Span);
	
	#[pattern("2")]
	#[derive(Clone, Parse, PartialEq, Eq, PartialOrd, Ord, Hash)]
	pub struct Two(Span);
	
	#[pattern("3")]
	#[derive(Clone, Parse, PartialEq, Eq, PartialOrd, Ord, Hash)]
	pub struct Three(Span);
	
	#[test]
	fn one_two_and_three() {

		#[derive(Parse, Debug)]
		struct OneTwoThree {
			
			one: One,
			two: Two,
			three: Three,
			span: Span
			
		}

		let ref mut state = State::from("123");

		let result = OneTwoThree::parse(state);

		match result {

			Ok(_) => println!("{state:#?}"),
			Err(error) =>{
				println!("{error}");
				println!("{state:#?}")
			
			} 
				

		}


	}

	#[test]
	fn one_two_or_three() {

		#[derive(Parse, Debug, Clone)]
		enum OneTwoThree {
			
			One(One),
			Two(Two),
			Three(Three)
			
		}
		
		let ref mut state = State::from(" 2 1 3 1 ");

		let result = Many::<OneTwoThree>::parse(state);

		assert!(result.is_ok());
		
		let ok = result.unwrap();
		
		println!("{:?}", ok)

	}
	

	#[pattern(r"[+]")]
	#[derive(Clone, Parse, PartialEq, Eq, PartialOrd, Ord, Hash)]
	pub struct Plus(Span);	
	
	#[pattern(r"[-]")]
	#[derive(Clone, Parse, PartialEq, Eq, PartialOrd, Ord, Hash)]
	pub struct Minus(Span);

	#[test]
	fn expression() {
	
		#[derive(Parse, Debug)]
		enum Operator {
			
			Plus(Plus),
			Minus(Minus),

		}
		
		#[pattern(r"\d+")]
		#[derive(Clone, Parse, PartialEq, Eq, PartialOrd, Ord, Hash)]
		pub struct Number(Span);
	
		#[derive(Parse, Debug)]
		struct Term {
			
			span: Span,
			operator_maybe: Maybe::<Operator>,
			number: Number,

		}
		
		let ref mut state = State::from(" 2 + 2 - 1 ");
		
		let result = Many::<Term>::parse(state); 
	
		println!("{result:#?}")
		
	}


	#[pattern(r"\b\w+\b")]
	#[derive(Clone, Parse, PartialEq, Eq, PartialOrd, Ord, Hash)]
	pub struct Word(Span);

	#[test]
	fn find_all() {
		
		let ref mut state = State::from("asdf \t \n batman");

		let results = Word::find_all(state);		
		let length = results.len();
		
		assert_eq!(length, 2);

	}
	
	#[test]
	fn list() {

		let ref mut state = State::from("parsing , is \t, cool");
		
		let ok = List::<Word>::parse(state).unwrap();
		
		assert_eq!(ok.len(), 3);

	}
	
}
