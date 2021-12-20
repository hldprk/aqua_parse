use super::*;

/// Implements `Parse` for a struct with a `pattern` attribute.
pub fn pattern_helper(identifier: Ident, options: Options) -> TokenStream {

	let pattern = options.pattern_maybe.unwrap();

	let whitespace_parse = match options.is_strict {

		// parses as many `Space` as possible, not failing if none are found  
		false => quote!{ let _ = Vec::<crate::whitespace::Whitespace>::parse(string, index); },
		
		true => quote!{}

	};

	quote_spanned! {identifier.span()=>

		impl std::fmt::Display for #identifier {

			fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		
				let Self(string) = self;
		
				write!(f, "{}", string)
		
			}

		}

		impl std::ops::Deref for #identifier {

			type Target = str;

			fn deref(&self) -> &Self::Target {

				let Self(string) = self;

				string.deref()

			}

		} 

		impl Parse for #identifier {

			fn label() -> String {

				stringify!(#identifier).to_string()

			}

			fn parse<'string>(string: &'string str, index: &mut usize) -> Result<'string, Self> {

				#whitespace_parse

				let label = Self::label();

				let mut error = Error::from(string);
				let description = &format!("Expected '{}' starting here", label);
				let message = Message::start(index.clone(), Level::Error, description);
				error += message;

				#whitespace_parse

				use regex::Regex;
				
				let regex_maybe = Regex::new(#pattern);

				if regex_maybe.is_err() { return Err(error); }

				let regex = regex_maybe.unwrap();
				
				let ref mut captures = regex.capture_locations();

				let match_result_maybe = regex.captures_read_at(captures, string, index.clone());
				
				if match_result_maybe.is_none() { return Err(error); }

				let match_result = match_result_maybe.unwrap();

				if match_result.start() != index.clone() { return Err(error); }

				let matched_string = &string[match_result.range()];

				*index += matched_string.len();

				Ok(Self(matched_string.into()))

			}

		}

	}

}