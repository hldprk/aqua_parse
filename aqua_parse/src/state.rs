
use std::fmt::Debug;
use super::*;

/// Information persisting between parses.
#[derive(Debug, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct State {
	
	pub(crate) page: Rc<Page>,
	pub(crate) index: usize,

}

impl State {

	/// Swaps out `self.page` and sets `self.index` to 0.
	pub fn swap_page(&mut self, page: Page) {

		self.page = Rc::new(page);
		self.index = 0;

	}

	/// `&self.page`
	pub fn page(&self) -> Rc<Page> {

		Rc::clone(&self.page)

	}

	/// The current index.
	pub fn index(&self) -> usize {

		self.index

	}

	/// A slice of the remaining `&str` starting at `self.index()`.
	pub fn remainder(&self) -> &str {
		
		&self.page[self.index()..]

	}

	/// The current line number.
	pub fn line_number(&self) -> usize {

		let span = Span::new(Rc::clone(&self.page), 0..self.index);

		let result = span.end().line();

		println!("{result}");

		result

	}

}

impl From<Page> for State {

	fn from(page: Page) -> Self {
		
		let page = Rc::new(page);
		let index = 0;

		Self { page, index }

	}

}

impl From<&str> for State {

	fn from(string: &str) -> Self {

		let page = Page::from(string);

		Self::from(page)

	}

} 