use proc_macro::TokenStream;
use proc_macro2::Span;
use quote::quote;
use syn::Ident;

/// implement of Command derive macro
pub(crate) fn impl_command_macro(ast: &syn::DeriveInput) -> TokenStream {
    let name = &ast.ident;
    let name_str = name.to_string();

    if !name_str.ends_with("Command") {
        panic!("[Command] macro needs the type name ends with `Command`");
    }

    let variant_str = &name_str[..name_str.len() - "Command".len()];
    let variant_ident = Ident::new(variant_str, Span::call_site());

    let generated = quote! {
        impl From<#name> for crate::command::Commands {
            fn from(v: #name) -> crate::command::Commands {
                crate::command::Commands::#variant_ident(v)
            }
        }
    };
    generated.into()
}
