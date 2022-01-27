use std::ops::BitAnd;
use std::ops::BitOr;

use super::*;

/// A helper that implements `Parse` for an `enum`.
pub(crate) fn enum_helper(identifier: Ident, parameters: Parameters, variants: Vec<Variant>, options: Options) -> TokenStream {
	
	let mut variant_parses = TokenStream::default();

	let parameters_with_bounds = parameters.parameters_with_bounds.clone();
	let parameters_without_bounds = parameters.parameters_without_bounds.clone();
	
	let whitespace_parse = match options.is_strict {

		// parses as many `Space` as possible, not failing if none are found  
		false => quote!{ let _ = crate::containers::Many::<crate::whitespace::Whitespace>::parse(state); },
		
		true => quote!{}

	};

	let mut spanned_match_arms = quote!();

	for variant in variants.iter() {
		
		let variant_identifier = variant.ident.clone();

		let variant_fields = variant.fields.clone();

		if variant_fields.len() != 1 || !matches!(variant_fields, Fields::Unnamed(_)) {

			return quote!(compile_error!("'Parse' enum variants must only have one unnamed field."));

		} 

		let field = variant_fields.iter().next().unwrap();

		let field_type = field.ty.clone();
		
		variant_parses.extend(quote!{

			#whitespace_parse

			match #field_type :: parse(cloned_state) {

				Ok(ok) => {

					state.clone_from(cloned_state);

					#whitespace_parse

					let result = Self :: #variant_identifier (ok);

					return Ok(result);

				},

				Err(error) => {
					
					cloned_state.clone_from(state);

					errors.push(error) 
				
				}

			};

		});

		spanned_match_arms.extend(quote_spanned!{variant.span()=>

			Self :: #variant_identifier (inner) => inner.span(),

		});

	}

	quote!{

		impl< #parameters_with_bounds > Parse for #identifier < #parameters_without_bounds > {

			fn span(&self) -> &Span {

				match self {

					#spanned_match_arms

				}

			}
			
			fn parse(state: &mut State) -> Result<Self> {

				let mut errors = Vec::default();

				let ref mut cloned_state = state.clone();

				#variant_parses

				let error = errors.iter().min().unwrap().clone();

				Err(error)

			}

		}

	}

}

