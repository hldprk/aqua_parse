use super::*;

use std::collections::BTreeMap;
use std::ops::Range;

/// A map from a `struct` field's name (`&'static str`) to a location (`Range<usize>`) in a `&str`.
pub type Peek = BTreeMap<&'static str, Range<usize>>;

/// This is only implemented for `Peek` to get around the rules for derive-macros.
impl Parse for Peek {

	fn parse<'string>(_: &'string str, _: &mut usize) -> Result<'string, Self> {
		
		Ok(Self::default())

	}

}