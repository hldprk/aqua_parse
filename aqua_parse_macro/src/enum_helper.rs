use proc_macro2::*;
use syn::*;
use quote::*;
use syn::spanned::Spanned;

pub fn enum_helper(identifier: Ident, data_enum: DataEnum, is_strict: bool) -> proc_macro2::TokenStream {
	
	// assignment statements for variables 
	// holding `Result` from each variant's parse
	let mut maybies = proc_macro2::TokenStream::default();
	
	// 'else if' statements that decide which variant wins
	let mut else_ifs = proc_macro2::TokenStream::default();

	let mut error_unwraps = proc_macro2::TokenStream::default();

	let mut all_variant_maybe_identifiers = Vec::default();

	let whitespace_parse = match is_strict {

		false => quote!{ let _ = Option::<Vec::<Whitespace>>::parse(position); },
		
		true => quote!{}

	};

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

					#whitespace_parse

					Ok(Self::#variant_identifier(parsed))

				}

			});

			error_unwraps.extend(quote_spanned!{variant.span()=>

				let #variant_error_identifier = #variant_maybe_identifier.unwrap_err();
				
				errors.push(#variant_error_identifier);

			});
		}

		else { 
			
			return quote_spanned! {variant.fields.span()=>
			
				compile_error!("only tuple-like variants with one field are allowed for 'Parse' enums") 
			
			};
			
		}
		
	}
	
	let mut impl_body = quote!{ 
		
		use std::borrow::BorrowMut;
	
		let start_position = position.clone();

	};
	
	impl_body.extend(whitespace_parse);
	impl_body.extend(maybies);
	
	impl_body.extend(else_ifs);

	// deciding which variant's error is chosen when none are `Ok` 
	// depends on which error's `Position` made the most progress after parsing  
	impl_body.extend(quote!{

		else {

			let mut errors = Vec::<Error>::default();
			
			#error_unwraps

			let cause = Box::from(errors.into_iter().max().unwrap());

			let error = Error::branch::<#identifier>(start_position, cause);

			Err(error)

		}

	});

	quote! {

		impl Parse for #identifier{

			fn parse(position: &mut Position) -> Result<Self> {

				if position.clone().next().is_none() {

					return Err(Error::unexpected_end::<#identifier>(position.clone()))

				}

				#impl_body

			}

		}

	}

}

