use proc_macro::TokenStream;
use quote::quote;

pub(crate) fn impl_intent_macro(ast: &syn::DeriveInput) -> TokenStream {
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
