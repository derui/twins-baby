use proc_macro::TokenStream;
use quote::quote;

/// implement of Command derive macro
pub(crate) fn impl_command_macro(ast: &syn::DeriveInput) -> TokenStream {
    let name = &ast.ident;

    if !name.to_string().ends_with("Command") {
        panic!("[Command] macro needs the type name ends with `Command`");
    }

    let generated = quote! {
        impl crate::command::Command for #name {
            fn id(&self) -> &CommandId {
                &(*self.id)
            }

            fn select_ref<T: Command + 'static>(&self) -> Option<&T> {
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
