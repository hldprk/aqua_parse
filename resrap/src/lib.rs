#![allow(unused_imports)]
#![allow(incomplete_features)]
#![allow(dead_code)]
#![allow(legacy_derive_helpers)]
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
mod whitespace;
mod spanned;
mod pattern;

pub use resrap_proc_macro::*;
pub use parse::*;
pub use spanned::*;
pub use position::*;
pub use error::*;
pub use padded::*;
pub use token::*;
pub use whitespace::*;
pub use pattern::*;

#[cfg(test)]
mod tests {

	use crate::*;

	type One = Token<"1">;
	type Two = Token<"2">;
	type Three = Token<"3">;


	#[test]
	fn sequence() -> Result<()> {
			
		#[derive(Parse, Debug)]
		pub struct Sequence {
			
			one: One,
			two: Two,
			three: Three,
			
		}

		let ref mut position = Position::from("123");

		let _ = Sequence::parse(position)?;

		Ok(())

	}

	#[test]
	fn tuple_like_sequence() -> Result<()> {

		#[padded]
		#[derive(Parse, Debug)]
		pub struct TupleLikeSequence(One, Two, Three);

		let ref mut position = Position::from(" \t 123".to_string());

		let _ = TupleLikeSequence::parse(position)?;

		Ok(())

	}

	#[test]
	fn padded_sequence() -> Result<()> {
		
		#[padded]
		#[derive(Parse, Debug)]
		pub struct PaddedSequence { 
			
			one: One,
			two: Two,
			three: Three,
			
		}
		
		let ref mut position = Position::from(" 1  2 \n\r3 \t");

		PaddedSequence::parse(position)?;

		Ok(())

	}

	#[test]
	fn choice() -> Result<()> {

		#[derive(Parse, Debug)]
		pub enum Choice {
	
			One(One),
			Two(Two),
			Three(Three),
	
		}

		let ref mut position = Position::from("312");

		Choice::parse(position)?;
		Choice::parse(position)?;
		Choice::parse(position)?;

		Ok(())

	}

	#[test]
	fn padded_choice() -> Result<()> {

		#[padded]
		#[derive(Parse, Debug)]
		pub enum PaddedChoice {
	
			One(One),
			Two(Two),
			Three(Three),
	
		}

		let ref mut position = Position::from(" 3 \t \n 1  2    ");

		PaddedChoice::parse(position)?;
		PaddedChoice::parse(position)?;
		PaddedChoice::parse(position)?;

		Ok(())

	}

	#[test]
	fn owned_parses() -> Result<()> {

		let ref mut position = Position::from("12");

		Box::<One>::parse(position)?;
		std::rc::Rc::<Two>::parse(position)?;

		Ok(())

	}

	#[test]
	fn pattern() -> Result<()> {

		let ref mut position = Position::from("1234asdf");

		type Number = Pattern<"[0-9]+">;

		let _ = Number::parse(position)?;

		Ok(())

	}

	

}
