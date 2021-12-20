use super::*;

/// Implements `Parse` for a `struct` with named fields. 
pub fn named_helper(identifier: Ident, fields: Fields, options: Options) -> TokenStream {

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

	for field in fields.clone() {

		let field_identifier = field.ident.clone().unwrap();
	
		let field_type = field.ty.clone();
		let peek_type = parse2::<Type>(quote!(Peek)).unwrap();
		
		// an identifier that holds the `Result` from that field's `parse` method
		let field_maybe = 
			Ident::new(	&format!("{field_identifier}_maybe"), proc_macro2::Span::call_site());
		
		// a fragment of the final return expression
		let return_fragment = quote!{ #field_identifier : #field_identifier, };

		return_variables.extend(return_fragment);
		
		if peek_type != field_type {

			field_parses.extend(quote_spanned!{field.span()=>

				start_index = index.clone();
			
				let #field_maybe = #field_type::parse(string, index);
				
				end_index = index.clone();
	
				if #field_maybe.is_err() {
					
					let mut error = #field_maybe.unwrap_err();
					let label = Self::label();
					let description = &format!("Expected '{}' starting here", label);
					let message = Message::start(index.clone(), Level::Error, description);
					
					error += message;
					
					return Err(error);
					
				}
				
				peek.insert(stringify!(#field_identifier), start_index..end_index);
				
				#whitespace_parse

				let #field_identifier = #field_maybe.unwrap();
	
			}.to_token_stream());
	

		}

		
	}

	let return_value = quote!{ Self { #return_variables } };

	quote_spanned! {identifier.span()=>

		impl Parse for #identifier {

			fn label() -> String {

				stringify!(#identifier).to_string()

			}

			fn parse<'string>(string: &'string str, index: &mut usize) -> Result<'string, Self> {

				use std::collections::BTreeMap;
				use std::ops::Range;

				#[allow(unused_assignments)]
				let mut start_index = 0;
				
				#[allow(unused_assignments)]
				let mut end_index = 0;
				
				#[allow(unused_assignments)]
				let mut peek = BTreeMap::<&'static str, Range<usize>>::default();

				#field_parses

				#[allow(unused_mut)]
				let mut result = Ok(#return_value);
				
				result

			}

		}

	}

}