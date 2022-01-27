use super::*;

/// A container for type parameters' tokens.
#[derive(Clone, Debug)]
pub(crate) struct Parameters {

	pub(crate) parameters_without_bounds: Punctuated<GenericParam, Comma>,
	pub(crate) parameters_with_bounds: Punctuated<GenericParam, Comma>

}

/// Separates type parameters into those with type bounds and those without. 
impl From<Punctuated<GenericParam, Comma>> for Parameters {

	fn from(other_parameters: Punctuated<GenericParam, Comma>) -> Self {

		let parameters_with_bounds = other_parameters.clone();
		let mut parameters_without_bounds = other_parameters.clone();
		
		// omits type bounds and defaults for `parameters`
		for parameter in &mut parameters_without_bounds {

			match parameter {

				GenericParam::Type(ref mut type_parameter) => {

					if type_parameter.colon_token.is_some() {

						type_parameter.colon_token = None;
						type_parameter.bounds = Default::default();

					}

					if type_parameter.eq_token.is_some() {

						type_parameter.eq_token = None;
						type_parameter.default = None;

					}

				},
				_ => continue

			}

		}

		Parameters { parameters_without_bounds, parameters_with_bounds }

	}

}