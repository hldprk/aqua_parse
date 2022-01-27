use super::*;

/// Appends an implementation of `Parse` to some `struct` or `enum` definition.
pub fn derive_helper(input: proc_macro2::TokenStream) -> TokenStream {
	
	let derive_input = syn::parse2::<DeriveInput>(input).unwrap();
	let identifier = derive_input.ident.clone();
	let data = derive_input.data.clone();
	let attributes = derive_input.attrs.clone();
	let parameters = Parameters::from(derive_input.generics.params.clone());

	let options = Options::from(attributes);

	// delegates a helper function based on data type
	match data {

		Data::Struct(data_struct) => {
			
			let fields = data_struct.fields;
		
			struct_helper(identifier, parameters, fields, options)

		},
		
		Data::Enum(data_enum) => {
	
			let variants = data_enum.variants.into_iter().collect();			

			enum_helper(identifier, parameters, variants, options)
				
		},
		
		Data::Union(_) => quote_spanned! {derive_input.span()=> 
		
			compile_error!("Unions can't derive 'Parse'.") 
		
		}

	}
		
}


