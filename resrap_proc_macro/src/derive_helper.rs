use proc_macro2::*;
use syn::*;
use quote::*;
use syn::spanned::Spanned;
use crate::*;

pub fn derive_helper(input: proc_macro2::TokenStream) -> proc_macro2::TokenStream {
	
	let derive_input = syn::parse2::<DeriveInput>(input).unwrap();
	let identifier = derive_input.ident.clone();
	let data = derive_input.data.clone();

	// checks for an outer attribute named "padded"
	let is_padded = 
		derive_input.attrs.iter()
		.find(|attribute|
			attribute.path.is_ident("padded"))
		.is_some();
	
	// delegates a helper function based on data type
	match data {

		Data::Struct(data_struct) => struct_helper(identifier, data_struct, is_padded),
		Data::Enum(data_enum) => enum_helper(identifier, data_enum, is_padded),
		Data::Union(_) => quote_spanned! {derive_input.span()=> 
		
			compile_error!("unions can't be 'Parse'") 
		
		}

	}
		
}


