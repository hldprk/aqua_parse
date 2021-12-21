
use super::*;

/// A helper for implementing `Parse` for structs.
pub(crate) fn struct_helper(identifier: Ident, parameters: Parameters, fields: Fields, options: Options) -> TokenStream {

	match fields {

		Fields::Named(_) => named_helper(identifier, parameters, fields, options),

		Fields::Unnamed(_) => {
		
			match options.pattern_maybe {

				Some(_) => pattern_helper(identifier, parameters, options),
				None => unnamed_helper(identifier, parameters, fields, options),

			}
		
		},

		Fields::Unit => {
			
			if options.literal_maybe.is_some() {

				literal_helper(identifier, options)

			}

			else {

				quote!{ compile_error! ("Unit structs must have 'literal' attribute.") }

			}

		}

	}

}