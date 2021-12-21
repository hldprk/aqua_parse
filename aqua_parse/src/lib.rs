#![allow(dead_code)]
#![allow(legacy_derive_helpers)]
#![allow(const_evaluatable_unchecked)]
#![allow(incomplete_features)]
#![feature(adt_const_params)]
#![feature(generic_const_exprs)]

//! A general-use parsing library made to use with the `aqua` crate.

/// Some iterable parsers.
pub mod containers;

mod found;
pub use found::*;

/// Definitions of whitespace parsers.
pub mod whitespace;

pub use regex;

mod parse;
pub use parse::*;

pub use aqua_parse_macro::*;

use aqua_error::*;


/// Type alias for `Result<T, aqua_error::Error<'string>>`.
pub type Result<'string, T> = std::result::Result<T, aqua_error::Error<'string>>;

#[cfg(test)]
mod tests {

	use crate::*;
	use crate::containers::*;

	#[test]
	fn one_two_and_three() {

		#[literal("1")]
		#[derive(Parse, Debug)]
		struct One;

		#[literal("2")]
		#[derive(Parse, Debug)]
		struct Two;

		#[literal("3")]
		#[derive(Parse, Debug)]
		struct Three;

		#[derive(Parse, Debug)]
		struct OneTwoThree {
			
			one: One,
			two: Two,
			three: Three,
			findings: Findings
			
		}

		let result = OneTwoThree::parse("1 2 3", &mut 0);

		match result {

			Ok(ok) => {
				
				let findings = ok.findings();

				println!("{findings:?}")
				
			},
			Err(error) => println!("{error}"),

		}

	}

	#[test]
	fn one_two_or_three() {

		#[derive(Parse, Debug, Clone)]
		enum OneTwoThree {
			
			#[literal("1")]
			One,
			#[literal("2")]
			Two,
			#[literal("3")]
			Three
			
		}

		let result = Exactly::<4, OneTwoThree>::parse(" 2 1 3   1 ", &mut 0);

		assert!(result.is_ok())

	}
	
	#[test]
	fn expression() {
	
		#[derive(Parse, Debug)]
		enum Operator {
			
			#[literal("*")]
			Multiply,
			#[literal("/")]
			Divide,
			#[literal("+")]
			Plus,
			#[literal("-")]
		 	Minus,
			 
		}
		
		#[pattern(r"\d+")]
		#[derive(Parse, Debug)]
		struct Number(pub String);

		#[derive(Parse, Debug)]
		enum Expression<P : Parse> {
			
			Binary(P, Operator, Box::<Expression<P>>),
			Number(P)
			
		}
		
		let ref mut index = 0;
		
		let result = Expression::<Number>::parse("2 * 2 \\ 1", index); 
		
		match result {
			
			Ok(ok) => println!("{ok:?}"),
			Err(error) => println!("{error}"),

		}
		
	}

	#[pattern(r"\b\w+")]
	#[derive(Parse, Debug, Hash)]
	pub(crate) struct Word(pub String);

	#[test]
	fn find_all() {
		
		let string = " asdf \t \n batman";
		let ref mut index = 0;

		let results = Word::find_all(string, index);		
		let length = results.len();
		
		assert_eq!(length, 2)

	}
	
	#[test]
	fn reverse_find() {
		
		let string = " asdf \t \n batman \t\r\n \t ";
		let ref mut index = string.len() - 1;

		let result = Word::find(string, index, Direction::Backward);		
	
		println!("{result:?}");

	}

	#[test]
	fn list() {

		let string = "parsing, \t is , cool";
		let ref mut index = 0;
		
		#[derive(Parse, Debug, Hash)]
		pub struct Test {
			
			pub(crate) words: List::<Word>
			
		}

		let ok = Test::parse(string, index).unwrap();

		assert_eq!(ok.words.len(), 3)

	}
	
}
