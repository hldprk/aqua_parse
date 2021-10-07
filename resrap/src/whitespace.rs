use crate::*;

/// Container for a whitespace `char`.
#[derive(Copy, Debug, Clone, PartialEq, Eq)]
pub struct Whitespace(pub char);

impl std::ops::Deref for Whitespace {
    
	type Target = char;

    fn deref(&self) -> &Self::Target { &self.0 }

}

impl Parse for Whitespace {

	fn parse(position: &mut Position) -> Result<Self> {
		
		let error = Error {

			identifier: "Whitespace".to_string(),
			position: position.clone(),
			cause: None

		};

		let cloned_next = position.clone().next();

		if cloned_next.is_none() { return Err(error); }

		let cloned_character = cloned_next.unwrap();

		if cloned_character.is_whitespace() {

			position.next();

			Ok(Self(cloned_character))
		
		} else {

			Err(error)

		} 

	}

}