#![allow(unused)]
use proc_macro2::TokenStream;
use quote::quote;
use syn::DeriveInput;

use darling::{
    ast::{Data, Fields, Style},
    FromAttributes, FromDeriveInput, FromField, FromVariant,
};
use syn::{Attribute, Generics, Ident, Type};

#[derive(Debug, FromDeriveInput)]

struct MyErrorInfo {
    ident: Ident,
    generics: Generics,
    data: Data<MyErrorVariantInfo, ()>,
}

#[derive(Debug, FromVariant)]
struct MyErrorVariantInfo {
    ident: Ident,
    fields: Fields<MyErrorFieldInfo>,
}

#[derive(Debug, FromField)]
#[darling(attributes(my_error))]
struct MyErrorFieldInfo {
    ident: Option<Ident>,
    from: bool,
    ty: Type,
}
pub fn process_my_error(input: DeriveInput) -> TokenStream {
    let MyErrorInfo {
        ident,
        generics,
        data: Data::Enum(variants),
    } = MyErrorInfo::from_derive_input(&input).unwrap()
    else {
        panic!("only supported enum")
    };
    let codes: Vec<_> = variants
        .into_iter()
        .map(|v| {
            let var = v.ident;
            let fields = v.fields;
            if fields.fields.len() != 1 {
                panic!("MyError only support 1 argument!");
            }
            let field = fields.fields.into_iter().next().unwrap();
            let ty = field.ty;
            let from = field.from;
            if !from {
                return quote! {};
            }
            if let Some(field_ident) = field.ident {
                quote! {
                    impl #generics From<#ty> for #ident #generics {
                        fn from(v: #ty) -> Self {
                            #ident::#var {
                                #field_ident: v
                            }
                        }
                    }
                }
            } else {
                quote! {
                    impl #generics From<#ty> for #ident #generics {
                        fn from(v: #ty) -> Self {
                            #ident::#var(v)
                        }
                    }
                }
            }
        })
        .collect();
    quote! {
        #(#codes)*
    }
}
