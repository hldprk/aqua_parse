#![allow(unused_imports)]
#![allow(incomplete_features)]
#![allow(dead_code)]
#![feature(iter_advance_by)]
#![feature(adt_const_params)]


//! A parsing library heavily leveraged by `proc_macro`.
//!
//! In `resrap`, syntax trees are defined by the structure of data types implmenting [Parse]. 
//!
//! Given `T : Parse`, a `resrap::Result<T>` is returned by passing a [Position] to `T::parse`.
//!
//! Along with the [Parse] trait, there's also a `derive` macro of the same name which 
//! automatically implements [Parse] for a `struct` or `enum`, given each of its members or variants also implement it.


mod parse;
mod position;
mod error;
mod padded;
mod token;

pub use resrap_proc_macro::Parse;
pub use resrap_proc_macro::*;

pub use parse::*;
pub use position::*;
pub use error::*;
pub use padded::*;
pub use token::*;

#[cfg(test)]
mod tests {

	use crate::*;

	type One = Token<"1">;
	type Two = Token<"2">;
	type Three = Token<"3">;
	
	
	#[derive(Parse, Debug)]
	pub struct Sequence {
		
		one: One,
		two: Two,
		three: Three,
		
	}
	
	#[padded]
	#[derive(Parse, Debug)]
	pub struct PaddedSequence { 
		
		one: One,
		two: Two,
		three: Three,
		
	}
	
	#[derive(Parse, Debug)]
	pub enum Choice {

		One(One),
		Two(Two),
		Three(Three),

	}

	#[test]
	fn sequence() {

		let ref mut position = Position::from("123");

		match Sequence::parse(position) {

			Ok(_) => (),
			Err(error) => println!("{}", error)

		}

	}

	#[test]
	fn padded_sequence() {

		let ref mut position = Position::from(" 1  2 4");

		match PaddedSequence::parse(position) {

			Ok(_) => (),
			Err(error) => println!("{}", error)

		}

	}

	#[test]
	fn choice() {

		let ref mut position = Position::from("312");

		match Choice::parse(position) {

			Ok(_) => (),
			Err(error) => println!("{}", error)

		}

		match Choice::parse(position) {

			Ok(_) => (),
			Err(error) => println!("{}", error)

		}

		match Choice::parse(position) {

			Ok(_) => (),
			Err(error) => println!("{}", error)

		}

	}

}
