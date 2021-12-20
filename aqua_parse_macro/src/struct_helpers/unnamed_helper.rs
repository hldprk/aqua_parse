use super::*;


/// A helper for implementing `Parse` for tuple-structs.
pub fn unnamed_helper(identifier: Ident, fields: Fields, options: Options) -> TokenStream {

	// each fields' parses
	let mut field_parses = TokenStream::default();

	// tokens for variables returned in final return statement
	let mut return_variables = TokenStream::default();

	let whitespace_parse = match options.is_strict {

		// parses as many `Space` as possible, not failing if none are found  
		false => quote!{ let _ = Vec::<crate::whitespace::Whitespace>::parse(string, index); },
		
		true => quote!{}

	};

	field_parses.extend(whitespace_parse.clone());

	for (field_number, field) in fields.clone().iter().enumerate() {

		let field_return_string = &format!("field_{field_number}");
		let field_return_identifier = Ident::new( field_return_string ,Span::call_site());
	
		let field_type = field.ty.clone();

		// an identifier that holds the `Result` from that field's `parse` method
		let field_maybe = 
		Ident::new(	&format!("{field_return_identifier}_maybe"), proc_macro2::Span::call_site());
		
		// a fragment of the final return expression
		let return_fragment = quote!{ #field_return_identifier, };
		
		return_variables.extend(return_fragment);

		field_parses.extend(quote_spanned!{field.span()=>

			let #field_maybe = #field_type::parse(string, index);

			if #field_maybe.is_err() {

				let mut error = #field_maybe.unwrap_err();
				let label = Self::label();
				let description = &format!("Expected '{}' starting here", label);
				let message = Message::start(index.clone(), Level::Error, description);

				error += message;

				return Err(error);

			}

			let #field_return_identifier = #field_maybe.unwrap();

			#whitespace_parse

		}.to_token_stream());

	}

	// the initialization of `Self` depends on the style of `struct` it is
	let return_value = quote!{ Self( #return_variables) };

	quote_spanned! {identifier.span()=>

		impl Parse for #identifier {

			fn label() -> String {

				stringify!(#identifier).to_string()

			}

			fn parse<'string>(string: &'string str, index: &mut usize) -> Result<'string, Self> {

				#field_parses

				#[allow(unused_mut)]
				let mut result = Ok(#return_value);
				
				result

			}

		}

	}

}