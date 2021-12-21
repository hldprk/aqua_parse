use std::ops::BitAnd;
use std::ops::BitOr;

use super::*;

/// A helper that implements `Parse` for an `enum`.
pub(crate) fn enum_helper(identifier: Ident, parameters: Parameters, variants: Vec<Variant>, options: Options) -> TokenStream {
	
	let mut implementation_body = TokenStream::default();

	let parameters_with_bounds = parameters.parameters_with_bounds.clone();
	let parameters_without_bounds = parameters.parameters_without_bounds.clone();
	
	for (variant_number, variant) in variants.iter().enumerate() {
		
		let variant_identifier = variant.ident.clone();

		let other_identifier = Ident::new(&format!("Other{variant_identifier}"), Span::call_site());
		
		let variant_maybe_identifier = 
			Ident::new( &format!("variant_{variant_number}_maybe"),Span::call_site());
	
		let variant_index_identifier = 
			Ident::new( &format!("variant_{variant_number}_index"),Span::call_site());

		let variant_fields = variant.fields.clone();

		let mut return_variables = TokenStream::default();

		for (field_number, field) in variant_fields.clone().iter().enumerate() {
			
			let return_variable: TokenStream = match variant_fields {
				
				Fields::Named(..) => {
					
					let field_identifier = field.ident.clone().unwrap();
					quote!{ #field_identifier : other_result.#field_identifier, }

				},
				Fields::Unnamed(..) => {

					let result_field_identifier = Index::from(field_number);

					quote!{ other_result.#result_field_identifier, } 
				
				}

				Fields::Unit => quote!{ #identifier }

			};

			return_variables.extend(return_variable);

		}

		// an `Options` is created from the variant's attributes
		let variant_attributes = variant.attrs.clone();
		let mut variant_options = Options::from(variant_attributes);
		variant_options.is_strict = variant_options.is_strict || options.is_strict;
		
		let return_value = match variant_fields {

			Fields::Named(..) => quote!{ #identifier::<#parameters_without_bounds>::#variant_identifier { #return_variables } },
			Fields::Unnamed(..) => quote!{ #identifier::<#parameters_without_bounds>::#variant_identifier ( #return_variables) },
			Fields::Unit => quote! { #identifier::<#parameters_without_bounds>::#variant_identifier } 
			
		};

		let variant_implementation = struct_helper(other_identifier.clone(), parameters.clone(), variant_fields.clone(), variant_options);

		implementation_body.extend(quote!{

			#[derive(Debug)]
			struct #other_identifier <#parameters_with_bounds> #variant_fields;
			#variant_implementation

			let ref mut #variant_index_identifier = index.clone();

			let #variant_maybe_identifier = #other_identifier::<#parameters_without_bounds>::parse(string, #variant_index_identifier); 

			if #variant_maybe_identifier.is_ok() {

				let other_result = #variant_maybe_identifier.unwrap();

				#[allow(unused_mut)]
				let mut result = Ok(#return_value); 

				index.clone_from(#variant_index_identifier);

				return result;

			} else {

				let mut error = #variant_maybe_identifier.unwrap_err();
				
				errors.push(error);

			}

		});

	}

	quote!{

		impl<#parameters_with_bounds> Parse for #identifier<#parameters_without_bounds> {

			fn parse<'string>(string: &'string str, index: &mut usize) -> Result<'string, Self> {

				let mut errors = Vec::default();

				#implementation_body

				let error = errors.iter().min().unwrap().clone();

				Err(error)

			}

		}

	}

}

