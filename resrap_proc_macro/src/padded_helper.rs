use quote::*;
use syn::*;

pub fn padded_helper(_: proc_macro2::TokenStream, item: proc_macro2::TokenStream) -> proc_macro2::TokenStream {

	let item_struct = syn::parse2::<ItemStruct>(item.clone()).unwrap();

	let type_identifier = item_struct.ident.clone();

	let mut output = quote!();

	let attributes: proc_macro2::TokenStream = 
		item_struct.attrs.clone().iter().map(
			|a|
				a.to_token_stream()).flatten().collect();
	
	let visibility = item_struct.vis.clone();
	let generics = item_struct.generics.clone();

	output.extend(quote!{

		#attributes
		#visibility struct #type_identifier #generics

	});

	let mut fields = quote!();
	
	for field in item_struct.fields {

		let field_identifier = field.ident;
		let field_type = field.ty;

		fields.extend(quote!{ #field_identifier: Padded::<#field_type>, });

	}

	output.extend(quote!{ {#fields}});

	output

}