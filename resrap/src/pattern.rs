use crate::*;
use regex::Regex;

/// Similar to [Token], except this will parse a `Regex` pattern.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Pattern<const PATTERN: &'static str> {

	matched_string: String

}

impl<const PATTERN: &'static str> Pattern<PATTERN> {

	/// Returns the `Regex` source passed to this type.
	pub fn pattern() -> &'static str { PATTERN }

	/// Returns the matched string.
	pub fn matched_string(&self) -> String { self.matched_string.clone() }

}

impl<const PATTERN: &'static str> Parse for Pattern<PATTERN> {

	fn parse(position: &mut Position) -> Result<Self> {
		
		let start_position = position.clone();

		let error = Error {

			identifier: PATTERN.to_string(),
			position: start_position.clone(),
			cause: None

		};

		let regex_maybe = Regex::new(PATTERN);

		if regex_maybe.is_err() { return Err(error); }

		let regex = regex_maybe.unwrap();

		let remainder = &position.source()[position.index()..];

		let regex_match_maybe = regex.find(remainder);

		if regex_match_maybe.is_none() { return Err(error); }

		let regex_match = regex_match_maybe.unwrap();

		if regex_match.start() != 0 { return Err(error); }

		let matched_string = regex_match.as_str().to_string();

		for _ in 0..matched_string.len() { position.next(); }

		Ok(Self { matched_string })

	}

}