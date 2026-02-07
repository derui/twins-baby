extern crate proc_macro;

use proc_macro::TokenStream;
use quote::quote;
use syn;

/// Derive [Id] to make cad-base's Id implementation
#[proc_macro_derive(Id)]
pub fn id_derive(input: TokenStream) -> TokenStream {
    let ast = syn::parse(input).unwrap();

    impl_id_macro(&ast)
}

fn impl_id_macro(ast: &syn::DeriveInput) -> TokenStream {
    let name = &ast.ident;
    let type_name = ast.ident.to_string();

    if !type_name.ends_with("Id") {
        panic!("[Id] macro needs the type name ends with `Id`");
    }
    
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
        
        impl Display for PlaneId {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(f, "{}{}", stringify!(#type_name) self.0)
            }
        }
    };
    generated.into()
}
