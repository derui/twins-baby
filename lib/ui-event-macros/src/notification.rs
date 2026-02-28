use proc_macro::TokenStream;
use quote::quote;

/// implement of Notification derive macro
pub(crate) fn impl_notification_macro(ast: &syn::DeriveInput) -> TokenStream {
    let name = &ast.ident;

    if !name.to_string().ends_with("Notification") {
        panic!("[Notification] macro needs the type name ends with `Notification`");
    }

    let generated = quote! {
        impl crate::notification::Notification for #name {
            fn correlation_id(&self) -> &CommandId {
                &(*self.correlation_id)
            }

            fn select_ref<T: Notification + 'static>(&self) -> Option<&T> {
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
