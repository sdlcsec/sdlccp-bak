use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput};

#[proc_macro_derive(RegisterSchema)]
pub fn register_schema(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = &input.ident;
    
    let output = quote! {
        inventory::submit! {
            crate::SchemaGenerator::new(stringify!(#name), || schemars::schema_for!(#name))
        }
    };
    
    output.into()
}