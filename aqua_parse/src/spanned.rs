use super::*;

use std::collections::BTreeMap;
use std::ops::Range;

/// A map from a `struct` field's name (`&'static str`) to a span (`Range<usize>`) in a `&str`.
/// 
/// An entry with the key `"self"` is automatically included encompassing the entire `struct` or `enum`.
pub type Spans = BTreeMap<&'static str, Range<usize>>;

/// A `Parse` object that can describe it's location in a `&str` through a `Spans`.
/// 
/// An implementation is automatically generated for types deriving `Parse`
/// when it has a field with the type `Spans`. 
pub trait Spanned : Parse {

	fn spans(&self) -> &Spans;

}

/// This is only implemented for `Spans` to get around the rules for derive-macros.
impl Parse for Spans {

	fn parse<'string>(_: &'string str, _: &mut usize) -> Result<'string, Self> {
		
		Ok(Self::default())

	}

}