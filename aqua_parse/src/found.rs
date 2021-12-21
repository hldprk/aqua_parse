use super::*;

use std::collections::BTreeMap;
use std::ops::Range;

/// A `Parse` object that can describe it's location in a `&str` through a `Findings`.
/// 
/// An implementation is automatically generated for types deriving `Parse`
/// when it has a field with the type `Findings`. 
pub trait Found : Parse{

	fn findings(&self) -> &Findings;

}

/// A map from a `struct` field's name (`&'static str`) to a location (`Range<usize>`) in a `&str`.
/// 
/// An entry with the key `"self"` is automatically included encompassing the entire `struct` or `enum`.
pub type Findings = BTreeMap<&'static str, Range<usize>>;

/// This is only implemented for `Peek` to get around the rules for derive-macros.
impl Parse for Findings {

	fn parse<'string>(_: &'string str, _: &mut usize) -> Result<'string, Self> {
		
		Ok(Self::default())

	}

}