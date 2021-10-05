use proc_macro2::TokenStream;
use syn::*;
use quote::*;
use syn::spanned::Spanned;

pub fn parse_derive_helper(input: proc_macro2::TokenStream) -> proc_macro2::TokenStream {
	
	let derive_input = syn::parse2::<DeriveInput>(input).unwrap();

	let identifier = derive_input.ident.clone();

	let data = derive_input.data.clone();

	match data {

		Data::Struct(data_struct) => struct_helper(identifier, data_struct),
		Data::Enum(data_enum) => enum_helper(identifier, data_enum),
		Data::Union(_) => quote_spanned! {derive_input.span()=> 
		
			compile_error!("incompatible data type for 'Parse'") 
		
		}

	}
		
}

pub fn struct_helper(identifier: Ident, data_struct: DataStruct) -> proc_macro2::TokenStream {

	let mut field_parses = proc_macro2::TokenStream::default();
	let mut field_identifiers = proc_macro2::TokenStream::default();

	for field in data_struct.fields {

		let field_identifier_maybe = field.ident.clone();

		if field_identifier_maybe.is_none() {

			field_parses.extend(quote_spanned!{field.span()=>

				compile_error!("only named members allowed for 'Parse' structs");

			});

			break;

		}

		let field_identifier = field_identifier_maybe.unwrap();

		let field_type = field.ty.clone();
		
		let field_maybe = 
			Ident::new(
				&format!("{}_maybe", field_identifier),
				proc_macro2::Span::call_site()
			);
			
		field_identifiers.extend(quote_spanned!{field.span()=>
		
			#field_identifier, 
		
		}.to_token_stream());

		field_parses.extend(quote_spanned!{field.span()=>

			let #field_maybe = #field_type::parse(position);

			if #field_maybe.is_err() {

				let error = Some(Box::new(#field_maybe.unwrap_err()));

				return Err(Error {
					
					identifier: stringify!(#identifier).to_string(),
					position: start_position,
					cause: error
				
				});

			}

			let #field_identifier = #field_maybe.unwrap();

		}.to_token_stream());

	}


	let quoted = quote_spanned! {identifier.span()=>

		impl Parse for #identifier {

			fn parse(position: &mut Position) -> Result<Self> {

				let start_position = position.clone();

				#field_parses

				Ok(Self { #field_identifiers })

			}

		}

	};

	quoted

}

pub fn enum_helper(identifier: Ident, data_enum: DataEnum) -> proc_macro2::TokenStream {
		
	let mut maybies = proc_macro2::TokenStream::default();
	let mut else_ifs = proc_macro2::TokenStream::default();
	let mut error_unwraps = proc_macro2::TokenStream::default();

	let mut all_variant_maybe_identifiers = Vec::default();

	let mut else_ifs_count = 0;

	for (i, variant) in data_enum.variants.iter().enumerate() {
		
		let variant_identifier = variant.ident.clone();
		
		let variant_maybe_identifier = 
			Ident::new( &format!("{}_maybe", variant_identifier),proc_macro2::Span::call_site());
		
		let variant_error_identifier = 
			Ident::new( &format!("{}_error", variant_identifier),proc_macro2::Span::call_site());

		all_variant_maybe_identifiers.push(variant_maybe_identifier.clone());

		if let Fields::Unnamed(fields) = variant.fields.clone() {
									
			let fields = fields.unnamed;

			if fields.len() != 1 {

				return quote_spanned! {variant.span()=>
			
					compile_error!("currently only single-field, tuple variants are allowed for 'Parse' enums") 
				
				};

			}

			let field = fields.first().unwrap().clone();

			let field_type =  field.ty;
	
			maybies.extend(quote!{
				
				let #variant_maybe_identifier = #field_type::parse(position.clone().borrow_mut());
			
			});

			let if_or_else_if = 
				if i == 0 { quote!(if) } 
				else { quote!(else if) };

			else_ifs.extend(quote_spanned!{variant.span()=>

				#if_or_else_if #variant_maybe_identifier.is_ok() {

					let parsed = #field_type::parse(position).unwrap();

					Ok(Self::#variant_identifier(parsed))

				}

			});

			else_ifs_count = i;

			error_unwraps.extend(quote_spanned!{variant.span()=>

				let #variant_error_identifier = #variant_maybe_identifier.unwrap_err();
				
				errors.push(#variant_error_identifier);

			});
		}

		else { 
			
			return quote_spanned! {variant.fields.span()=>
			
				compile_error!("currently only single-field tuple-like variants are allowed for 'Parse' enums") 
			
			};
			
		}
		
	}
	
	let mut impl_body = quote!(use std::borrow::BorrowMut;);
	
	impl_body.extend(maybies);
	
	if else_ifs_count == 0 {
				
		return quote! {
		
			compile_error!("'Parse' enum must have at least one variant"); 
		
		};

	} 
	
	else {

		impl_body.extend(else_ifs);

	}



	impl_body.extend(quote!{

		else {

			let mut errors = Vec::<Error>::default();
			
			#error_unwraps

			let error = errors.into_iter().max().unwrap();

			let choice_error = Error {

				identifier: String::from("Choice"),
				position: error.position.clone(),
				cause: Some(Box::new(error.clone()))

			};

			Err(choice_error)

		}

	});

	let output = quote! {

		impl Parse for #identifier{

			fn parse(position: &mut Position) -> Result<Self> {

				#impl_body

			}

		}

	};

	output

}
