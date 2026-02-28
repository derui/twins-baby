extern crate proc_macro;

use proc_macro::TokenStream;
use quote::quote;

/// Derive [Intent] to make Intent
#[proc_macro_derive(Intent)]
pub fn intent_derive(input: TokenStream) -> TokenStream {
    let ast = syn::parse(input).unwrap();

    impl_intent_macro(&ast)
}

fn impl_intent_macro(ast: &syn::DeriveInput) -> TokenStream {
    let name = &ast.ident;

    if !name.to_string().ends_with("Intent") {
        panic!("[Intent] macro needs the type name ends with `Intent`");
    }

    let generated = quote! {
        impl crate::intent::Intent for #name {
            fn select_ref<T: Intent + 'static>(&self) -> Option<&T> {
                let type_id = std::any::TypeId::of::<T>();
                let current = std::any::TypeId::of::<#name>();

                if type_id == current {
                    let current: Box<&dyn std::any::Any> = Box::new(self);
                    current.downcast_ref::<T>()
                } else {
                    None
                }
            }
        }
    };
    generated.into()
}

/// Derive [Command] to make Command
#[proc_macro_derive(Command)]
pub fn command_derive(input: TokenStream) -> TokenStream {
    let ast = syn::parse(input).unwrap();

    impl_command_macro(&ast)
}

fn impl_command_macro(ast: &syn::DeriveInput) -> TokenStream {
    let name = &ast.ident;

    if !name.to_string().ends_with("Command") {
        panic!("[Command] macro needs the type name ends with `Command`");
    }

    let generated = quote! {
        impl crate::command::Command for #name {}
    };
    generated.into()
}
