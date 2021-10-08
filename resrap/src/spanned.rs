use std::ops::Index;
use std::ops::Range;
use std::ops::RangeBounds;
use std::rc::Rc;
use ansi_term::*;
use crate::*;


/// A [Position] with length.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Span {

	position: Position,
	length: usize

}

impl Span {

	pub fn position(&self) -> Position { self.position.clone() }
	
	pub fn length(&self) -> usize { self.length }
	
	pub fn slice(&self) -> &str {

		&self.position.source()[self.position.index() .. self.position.index() + self.length]

	} 

}

/// A [Parse] wrapper for a [Parse] value and a [Span].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Spanned<P : Parse> {

	value: P,
	span: Span

}

impl<P : Parse> Spanned<P> {

	pub fn value(&self) -> &P { &self.value }
	pub fn span(&self) -> &Span { &self.span }

}

impl<P : Parse> Parse for Spanned<P> {

	fn parse(position: &mut Position) -> Result<Self> {
		
		let start_position = position.clone();

		let value = P::parse(position)?;

		let length = position.index();

		let span = Span {

			position: start_position,
			length

		};

		Ok(Self { value, span })

	}

}