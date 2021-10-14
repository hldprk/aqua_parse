use std::borrow::BorrowMut;

use crate::*;

/// Interface for parsing from *values* of [`Parse`] types, as opposed to the types themselves.
pub trait ParseValue : Parse + PartialEq {

	/// Calls `Self::parse` and only returns `Ok(self)` if `self == result`.
	fn parse_value(&self, position: &mut Position) -> Result<Self>
	where Self : PartialEq + Clone {

		let self_maybe = Self::parse(position.clone().borrow_mut());

		match self_maybe {

			Ok(ok) => {

				use std::any::type_name;

				let error = Error {

					identifier: type_name::<Self>().to_string(),
					position: position.clone(),
					cause: None

				};

				if ok != self.clone() { Err(error) }
				else { Ok(ok) }

			},

			Err(error) => Err(error)

		}

	}

}

impl<P : Parse + PartialEq> ParseValue for P {}