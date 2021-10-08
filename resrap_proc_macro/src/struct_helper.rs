use proc_macro2::*;
use syn::*;
use quote::*;
use syn::spanned::Spanned;

pub fn struct_helper(identifier: Ident, data_struct: DataStruct, is_padded: bool) -> proc_macro2::TokenStream {

	// `TokenStream` containing each fields parses
	let mut field_parses = TokenStream::default();

	// tokens for variables returned in final return statement
	let mut return_variables = TokenStream::default();
	
	// for naming tuple fields
	let mut n = 0;

	let whitespace_parse = match is_padded {

		// parses as many whitespace characters as possible, not failing if none are found  
		true => quote!{ let _ = Option::<Vec::<Whitespace>>::parse(position); },
		
		false => quote!{}

	};

	field_parses.extend(whitespace_parse.clone());

	for field in data_struct.fields.clone() {

		// the identifier used in the `Parse` implementation depends on the struct style
		let field_identifier = match data_struct.fields {

			Fields::Named(..) => field.ident.clone().unwrap(),
			Fields::Unnamed(..) => Ident::new( &format!("field_{}", n) ,Span::call_site()),
			Fields::Unit => Ident::new( &n.to_string(),Span::call_site())
	
		};
		
		n += 1;
	
		let field_type = field.ty.clone();
		
		let field_maybe = 
			Ident::new(
				&format!("{}_maybe", field_identifier),
				proc_macro2::Span::call_site()
			);
			
		return_variables.extend(quote_spanned!{field.span()=>
		
			#field_identifier, 
		
		}.to_token_stream());

		field_parses.extend(quote_spanned!{field.span()=>

			let #field_maybe = #field_type::parse(position);

			if #field_maybe.is_err() {

				let cause = Some(Box::new(#field_maybe.unwrap_err()));

				let error = Error {
					
					identifier: stringify!(#identifier).to_string(),
					position: start_position,
					cause
				
				};

				return Err(error);

			}

			let #field_identifier = #field_maybe.unwrap();

			#whitespace_parse

		}.to_token_stream());

	}

	let return_value = match  data_struct.fields {

		Fields::Named(..) => quote!{ Self { #return_variables } },
		Fields::Unnamed(..) => quote!{ Self( #return_variables) },
		Fields::Unit => quote!{ Self }

	};
 
	let quoted = quote_spanned! {identifier.span()=>

		impl Parse for #identifier {

			fn parse(position: &mut Position) -> Result<Self> {

				let start_position = position.clone();

				#field_parses

				Ok(#return_value)

			}

		}

	};

	quoted

}