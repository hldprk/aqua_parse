use super::*;

/// Implements `Parse` for a unit struct with a `literal` attribute.
pub fn literal_helper(identifier: Ident, options: Options) -> TokenStream {

	let literal = options.literal_maybe.unwrap();

	let whitespace_parse = match options.is_strict {

		// parses as many `Space` as possible, not failing if none are found  
		false => quote!{ let _ = Vec::<crate::whitespace::Whitespace>::parse(string, index); },
		
		true => quote!{}

	};

	quote_spanned! {identifier.span()=>

		impl Parse for #identifier {

			fn label() -> String {

				stringify!(#identifier).to_string()

			}

			fn parse<'string>(string: &'string str, index: &mut usize) -> Result<'string, Self> {

				#whitespace_parse

				let literal = #literal;
				let remainder = &string[index.clone()..];

				if remainder.starts_with(literal) {

					*index += literal.len();

					#whitespace_parse

					Ok(Self)

				} else {
				
					let mut error = Error::from(string);
					let label = Self::label();
					let description = &format!("Expected '{}' starting here", label);
					let message = Message::start(index.clone(), Level::Error, description);
					error += message;
	
					return Err(error);

				}
				
			}

		}

	}

}