
use super::*;



/// Implements `Parse` for a `struct`
pub(crate) fn struct_helper(identifier: Ident, parameters: Parameters, fields: Fields, options: Options) -> TokenStream {
	
	let span_type = parse2::<Type>(quote!(Span)).unwrap();

	let span_field_maybe = fields.iter().find(|field| field.ty == span_type);
	
	let has_span_field = span_field_maybe.is_some();
	
	if !has_span_field {

		return quote!(compile_error!("'Parse' struct must have 'Span' field."));

	}

	let has_pattern = options.pattern_maybe.is_some();

	if fields.len() == 1 && has_pattern && has_span_field {

		return token_helper(identifier, options);

	}

	// tokens for each fields' parses
	let mut field_parses = TokenStream::default();

	// tokens for variables returned in final return statement
	let mut return_fragments = TokenStream::default();

	let has_parameters = parameters.parameters_with_bounds.len() > 0;
	let parameters_with_bounds = parameters.clone().parameters_with_bounds;
	let parameters_without_bounds = parameters.parameters_without_bounds;

	let description_method = match options.clone().description_maybe {

		Some(description) => quote_spanned! {description.span()=>
			
			fn description() -> String {

				format!(#description)

			}
		
		},

		None => quote!()

	};

	let label_method = match options.clone().label_maybe {

		Some(literal) => quote_spanned! {literal.span()=>
			
			fn label() -> String {

				#literal.to_string()

			}
		
		},

		None => {
			
			let generics = 
			if has_parameters { quote!(<#parameters_without_bounds>) }
			else { quote!() };

			quote!{
			
				fn label() -> String {

					let identifier = stringify!(#identifier);

					let generics = stringify!(#generics);

					return format!("{}{}", identifier, generics);

				}
			
			}
		
		},

	};

	let whitespace_parse = match options.is_strict {

		// parses as many `Space` as possible, not failing if none are found  
		false => quote!{ let _ = crate::containers::Many::<crate::whitespace::Whitespace>::parse(state); },
		
		true => quote!{}

	};

	let cloned_whitespace_parse = match options.is_strict {

		// parses as many `Space` as possible, not failing if none are found  
		false => quote!{ let _ = crate::containers::Many::<crate::whitespace::Whitespace>::parse(cloned_state); },
		
		true => quote!{}

	};

	let mut span_identifier = quote!();
	let mut span_accessor = quote!();

	for error_type in &options.error_types {

		field_parses.extend(quote_spanned!{error_type.span()=> 
			
			#cloned_whitespace_parse

			match #error_type::parse(cloned_state) {

				Ok(ok) => {

					let page = cloned_state.page();
					let span = ok.span().clone();
					let error = #error_type::error(span, Level::Error);

					return Err(error)

				}

				Err(_) => {
					
					cloned_state.clone_from(state);
					#cloned_whitespace_parse

				}

			}

		});

	}

	for (field_number, field) in fields.iter().enumerate() {

		let field_type = field.ty.clone();

		let is_span_field = field_type == span_type;

		let ref field_identifier_string = match fields {

			Fields::Named(_) => field.ident.clone().unwrap().to_string(),
			Fields::Unnamed(_) => format!("field_{field_number}"),
			Fields::Unit => "".to_string()

		};

		let field_identifier = Ident::new(field_identifier_string, field.span());

		let field_accessor = match &fields {

			Fields::Named(_) => field_identifier.to_token_stream(),
			Fields::Unnamed(_) => Index::from(field_number).to_token_stream(),
			Fields::Unit => field_identifier.to_token_stream()

		};

		if is_span_field {

			span_identifier = field_identifier.clone().to_token_stream();
			span_accessor = field_accessor.clone().to_token_stream();

		}

		else {

			field_parses.extend(quote_spanned!{field.span()=>
			
				#whitespace_parse
	
				let #field_identifier = match #field_type::parse(state) {
					
					Ok(ok) => { 
					
						end_index = state.index();
						ok 
					
					},

					Err(lower_error) => {

						end_index = lower_error.span().as_ref().unwrap().end().index();
						let range = start_index..end_index;
						let span = Span::new(page, range);
						let mut error = Self::error(span, Level::Error);
						error += lower_error;

						return Err(error);

					}
					
				};
		
				#whitespace_parse
	
			});

		}

		// a fragment of the final return expression
		let return_fragment = match fields {

			Fields::Named(_) => quote!{ #field_identifier, },
			Fields::Unnamed(_) => quote!{ #field_identifier, },
			Fields::Unit => quote!{}
	
		}; 

		return_fragments.extend(return_fragment);

	}

	let return_value = match fields {
		
		Fields::Named(_) => quote!{ #identifier { #return_fragments } },
		Fields::Unnamed(_) => quote!{ #identifier ( #return_fragments ) },
		Fields::Unit => quote! { #identifier }

	};
	
	quote_spanned! {identifier.span()=>

		impl<#parameters_with_bounds> Parse for #identifier<#parameters_without_bounds> {

			fn span(&self) -> &Span {

				&self.#span_accessor

			}

			#label_method

			#description_method

			fn parse(state: &mut State) -> Result<#identifier<#parameters_without_bounds>> {

				#![allow(unused_assignments)]

				let start_index = state.index();
				let mut end_index = state.index();
				let page = state.page();
				
				#[allow(unused_variables)]
				let ref mut cloned_state = state.clone();
				
				#field_parses
				
				let range = start_index..end_index;
				let #span_identifier = Span::new(page, range);
				
				Ok(#return_value)
				
			}

		}

	}

}