use crate::*;

/// Given `P : Parse`, parses `Option<Vec<Whitespace>>` before and after parsing `P`, then returns `P`.
#[derive(Debug)]
pub struct Padded<P : Parse>(pub P);

impl<P: Parse> std::ops::Deref for Padded<P> {

    type Target = P;

    fn deref(&self) -> &Self::Target { &self.0 }

}

impl<P : Parse> Parse for Padded<P> {

	fn parse(position: &mut Position) -> Result<Self> {
		
		let _ = Option::<Vec::<Whitespace>>::parse(position);
		
		let result = P::parse(position);
		
		let _ = Option::<Vec::<Whitespace>>::parse(position);

		match result {
			
			Ok(ok) => Ok(Padded(ok)),
			Err(error) => Err(error)
  
		}

	}

}

impl<T> Clone for Padded<T> where T : Parse + Clone {

	fn clone(&self) -> Self {
		
		Self(self.0.clone())

	}

}

impl<T> Copy for Padded<T> where T : Parse + Copy {}

impl<T> PartialEq for Padded<T> where T : Parse + PartialEq {

	fn eq(&self, other: &Self) -> bool {
		
		self.0 == other.0

	}
	
}

impl<T> Eq for Padded<T> where T : Parse + Eq {}

impl<T> PartialOrd for Padded<T> where T : Parse + PartialOrd {

	fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
		
		self.0.partial_cmp(&other.0)
		
	}

}

impl<T> Ord for Padded<T> where T : Parse + Ord {

	fn cmp(&self, other: &Self) -> std::cmp::Ordering {
		
		self.0.cmp(&other.0)

	}

}