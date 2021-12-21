
use super::*;

/// Implements `Parse` for a `struct` with named fields. 
pub(crate) fn named_helper(identifier: Ident, parameters: Parameters, fields: Fields, options: Options) -> TokenStream {

	// tokens for each fields' parses
	let mut field_parses = TokenStream::default();

	// tokens for variables returned in final return statement
	let mut return_variables = TokenStream::default();

	// the member name for this struct's `Findings` if there is one
	let mut findings_identifier_maybe = None;

	let parameters_with_bounds = parameters.parameters_with_bounds;
	let parameters_without_bounds = parameters.parameters_without_bounds;
	
	let whitespace_parse = match options.is_strict {

		// parses as many `Space` as possible, not failing if none are found  
		false => quote!{ let _ = Vec::<crate::whitespace::Whitespace>::parse(string, index); },
		
		true => quote!{}

	};

	field_parses.extend(whitespace_parse.clone());

	for field in fields.clone() {

		let field_identifier = field.ident.clone().unwrap();

		let field_type = field.ty.clone();
		let findings_type = parse2::<Type>(quote!(Findings)).unwrap();
		let is_not_findings = findings_type != field_type;
		
		// an identifier that holds the `Result` from that field's `parse` method
		let field_maybe = 
			Ident::new(	&format!("{field_identifier}_maybe"), proc_macro2::Span::call_site());
		
		// a fragment of the final return expression
		let return_fragment = quote!{ #field_identifier : #field_identifier, };

		return_variables.extend(return_fragment);

		if is_not_findings {

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
				
				findings.insert(stringify!(#field_identifier), start_index..end_index);
				
				#whitespace_parse

				let #field_identifier = #field_maybe.unwrap();
	
			}.to_token_stream());

		} 

		else {

			findings_identifier_maybe = Some(field_identifier)

		}
		
	}

	let return_value = quote!{ Self { #return_variables } };

	let found_implementation = match findings_identifier_maybe {

		Some(findings_identifier) => {quote! {

			impl<#parameters_with_bounds> Found for #identifier<#parameters_without_bounds> {

				fn findings(&self) -> &Findings {

					&self.#findings_identifier

				}

			}

		}},

		None => quote! {}

	};

	quote_spanned! {identifier.span()=>

		#found_implementation

		impl<#parameters_with_bounds> Parse for #identifier<#parameters_without_bounds> {

			fn label() -> String {

				stringify!(#identifier<#parameters_without_bounds>).to_string()

			}

			fn parse<'string>(string: &'string str, index: &mut usize) -> Result<'string, Self> {

				use std::collections::BTreeMap;
				use std::ops::Range;

				#[allow(unused_assignments)]
				let mut start_index = index.clone();
				
				#[allow(unused_assignments)]
				let mut end_index = index.clone();
				
				#[allow(unused_assignments)]
				let mut findings = BTreeMap::<&'static str, Range<usize>>::default();

				let self_start_index = index.clone();
				
				#field_parses

				let self_end_index = index.clone();
				
				findings.insert("self", self_start_index..self_end_index);

				#[allow(unused_mut)]
				let mut result = Ok(#return_value);
				
				result

			}

		}

	}

}