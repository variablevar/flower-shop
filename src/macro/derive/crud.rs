extern crate proc_macro;

use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput};

#[proc_macro_derive(Crud)]
pub fn generate_functions(input: TokenStream) -> TokenStream {
    // Parse the input tokens
    let ast = parse_macro_input!(input as DeriveInput);
    let name = &ast.ident;

    // Generate the output tokens
    let gen = quote! {
        impl #name {
                // Get _id
                pub fn object_id(id: &str) -> ObjectId {
                    ObjectId::from_str(&id).expect("Invalid Product Id")
                }

        }
    };

    // Return the generated implementation
    gen.into()
}
