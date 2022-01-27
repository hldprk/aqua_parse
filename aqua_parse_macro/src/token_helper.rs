use super::*;

/// Implements `Parse` and `Token` for a `struct`
pub(crate) fn token_helper(identifier: Ident, options: Options) -> TokenStream {

	let pattern = options.pattern_maybe.clone().unwrap();

	let label_method = match options.clone().label_maybe {

		Some(literal) => quote_spanned! {literal.span() =>
			
			fn label() -> String {

				#literal.to_string()

			}
		
		},

		None => {

			quote!{
			
				fn label() -> String {

					stringify!(#identifier).to_string()

				}
			
			}
		
		},

	};
	
	let whitespace_parse = match options.is_strict {

		// parses as many `Space` as possible, not failing if none are found  
		false => quote!{ let _ = crate::containers::Many::<crate::whitespace::Whitespace>::parse(state); },
		
		true => quote!{}

	};

	let description_method = match options.description_maybe {

		Some(error_literal) => quote!{

			fn description() -> String {

				format!(#error_literal)

			}

		},

		None => quote!()
		
	};

	quote_spanned!{identifier.span()=>

		impl std::fmt::Debug for #identifier {

			fn fmt(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {

				let self_string = stringify!(#identifier);

				write!(formatter, "{}", self_string)

			}

		}

		impl std::ops::Deref for #identifier {

			type Target = str;

			fn deref(&self) -> &Self::Target {

				self.0.slice()

			}

		} 

		impl Parse for #identifier {
			
			fn span(&self) -> &Span {

				&self.0

			}
			
			#label_method

			#description_method

			fn parse(state: &mut State) -> Result<Self> {
				
				let page = state.page();
				let start_index = state.index();
				let mut end_index = state.index() + 1;
				let error_range = start_index .. end_index;
				let error_span = Span::new(page,  error_range);
				let error = Self::error(error_span, Level::Error);
		
				if state.index() >= state.page().len() { return Err(error); } 
		
				#whitespace_parse

				let pattern = #pattern;
						
				let page  = state.page();
				let string = page.deref();
		
				let regex_maybe = Regex::new(pattern);
				
				if regex_maybe.is_err() { return Err(error); }
				
				let regex = regex_maybe.unwrap();
				
				let ref mut captures = regex.capture_locations();
		
				let match_result_maybe 
					= regex.captures_read_at(captures, string, state.index());
				
				if match_result_maybe.is_none() { return Err(error); }
				
				let match_result = match_result_maybe.unwrap();
				
				if match_result.start() != state.index()  { return Err(error); }
				
				let matched_string = &string[match_result.range()];
						
				let progress = matched_string.len();
				
				state.index += progress;
				
				#whitespace_parse

				end_index = state.index();
				let range = start_index .. end_index;
				let span = Span::new(page, range);

				Ok(Self(span) )

			}

		}
	
	}

}