use darling::{ast::Data, FromDeriveInput, FromField};
use proc_macro2::TokenStream;
use quote::quote;
use syn::{DeriveInput, Generics, Ident};

#[derive(FromDeriveInput, Debug)]
struct AutoDebugInfo {
    ident: Ident,
    generics: Generics,
    data: Data<(), AutoDebugFieldInfo>,
}

#[derive(FromField, Debug)]
#[darling(attributes(debug))]
struct AutoDebugFieldInfo {
    ident: Option<Ident>,
    #[darling(default)]
    skip: bool,
}

pub fn process_auto_debug(input: DeriveInput) -> TokenStream {
    let AutoDebugInfo {
        ident,
        generics,
        data: Data::Struct(fields),
    } = AutoDebugInfo::from_derive_input(&input).unwrap()
    else {
        panic!("AutoDeref only works on structs!");
    };
    let fields: Vec<_> = fields
        .into_iter()
        .filter_map(|f| {
            if f.skip {
                None
            } else {
                let ident = f.ident.unwrap();
                Some(quote! {
                    .field(stringify!(#ident), &self.#ident)
                })
            }
        })
        .collect();
    quote! {
        impl #generics ::core::fmt::Debug for #ident #generics {
            #[inline]
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                f.debug_struct(stringify!(#ident))
                #(#fields)*
                .finish()

            }
        }
    }
}
