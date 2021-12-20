use super::*;
use std::hash::*;

/// Comma-separated list, where trailing comma is optional.
#[derive(Debug, Clone, PartialEq, PartialOrd, Default)]
pub struct List<P : Parse>(Vec<P>);

impl<P : Parse> Deref for List<P> {

	type Target = [P];

	fn deref(&self) -> &Self::Target {
		
		self.0.deref()

	}
	
}

impl<P: Hash + Parse> Hash for List<P> {

	fn hash<H: Hasher>(&self, state: &mut H) {
       
		Hash::hash(&**self, state)
    
	}

}


impl<P : Parse> Parse for List<P> {

	fn parse<'string>(string: &'string str, index: &mut usize) -> Result<'string, Self> {
		
		#[literal(",")]
		#[derive(Debug, Parse, PartialEq, Eq, PartialOrd, Ord)]
		pub struct Comma;

		let mut content = Vec::default();

		loop {

			if P::can_parse(string, index) {

				let ok = P::parse(string, index)?;

				content.push(ok);

				if Comma::can_parse(string, index) {

					let _ = Comma::parse(string, index)?;

				}

				else { break; }

			}

			else { break; }
	
		}

		Ok(Self(content))

	}

}