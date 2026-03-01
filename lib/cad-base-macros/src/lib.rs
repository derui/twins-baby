extern crate proc_macro;

use proc_macro::TokenStream;
use quote::quote;

/// Derive [Id] to make cad-base's Id implementation
#[proc_macro_derive(MakeId)]
pub fn id_derive(input: TokenStream) -> TokenStream {
    let ast = syn::parse(input).unwrap();

    impl_id_macro(&ast)
}

fn impl_id_macro(ast: &syn::DeriveInput) -> TokenStream {
    let name = &ast.ident;
    let mut type_name = ast.ident.to_string();

    if !type_name.ends_with("Id") {
        panic!("[Id] macro needs the type name ends with `Id`");
    }
    // trim name
    type_name = type_name[0..(type_name.len() - 2)].to_string();

    let generated = quote! {
        impl #name {
            /// Creates a new [#name].
            pub fn new(id: u64) -> Self {
                #name(id)
            }
        }

        impl From<u64> for #name {
            fn from(id: u64) -> Self {
                #name(id)
            }
        }

        impl From<#name> for u64 {
            fn from(value: #name) -> Self {
                value.0
            }
        }

        impl std::fmt::Display for #name {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(f, "{}{}", #type_name, self.0)
            }
        }
    };
    generated.into()
}
