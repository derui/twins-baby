mod command;
mod intent;

extern crate proc_macro;

use proc_macro::TokenStream;

use crate::{command::impl_command_macro, intent::impl_intent_macro};

/// Derive [Intent] to make Intent
#[proc_macro_derive(Intent)]
pub fn intent_derive(input: TokenStream) -> TokenStream {
    let ast = syn::parse(input).unwrap();

    impl_intent_macro(&ast)
}

/// Derive [Command] to make Command
#[proc_macro_derive(Command)]
pub fn command_derive(input: TokenStream) -> TokenStream {
    let ast = syn::parse(input).unwrap();

    impl_command_macro(&ast)
}
