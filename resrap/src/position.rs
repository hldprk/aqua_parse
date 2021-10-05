
/// An `Iterator` over a string being parsed.
///
/// Can be initialized by some `Display` type by `Position::from`.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Position {

	source: String,
	index: usize,

}

impl<T : std::fmt::Display> From<T> for Position {

	fn from(t : T) -> Self {
		
		let index = 0;
		let source = t.to_string();

		Position { source, index }

	}

}

impl Iterator for Position {

	type Item = char;

	fn next(&mut self) -> Option<Self::Item> {
		
		let character_maybe = self.source.clone().chars().nth(self.index);

		if character_maybe.is_some() { self.index += 1; }

		character_maybe

	}

}

impl Position {

	/// 1-indexed, current column within the input string.
	pub fn column(&self) -> usize {

		let mut column  = 1;

		for (i, c) in self.source.clone().char_indices() {

			if i >= self.index { break; }
			else if c == '\n' { column = 1; }
			else { column += 1; }

		}

		column

	}

	/// Returns remaining string starting at `self.index()`.
	pub fn remainder(&self) -> &str {

		&self.source[self.index..]

	}

	/// Returns the input string this [Position] was initialized wit
	pub fn source(&self) -> &str {

		&self.source

	}

	/// 0-indexed, current index within the input string.
	pub fn index(&self) -> usize {

		self.index

	}

	/// 1-indexed, current column within the input string.
	pub fn row(&self) -> usize {
		
		let mut row  = 1;

		for (i, c) in self.source.clone().char_indices() {

			if i >= self.index { break; }
			else if c == '\n' { row += 1; }

		}

		row

	}

}


impl PartialOrd for Position {

	fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
		
		self.index.partial_cmp(&other.index)

	}

}

impl Ord for Position {

	fn cmp(&self, other: &Self) -> std::cmp::Ordering {
		
		self.index.cmp(&other.index)

	}

}