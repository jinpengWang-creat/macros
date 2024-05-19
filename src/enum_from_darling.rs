use darling::{
    ast::{Data, Fields, Style},
    FromDeriveInput, FromField, FromVariant,
};
use proc_macro2::TokenStream;
use quote::quote;
use syn::DeriveInput;

#[derive(Debug, FromDeriveInput)]
struct EnumFromDarling {
    ident: syn::Ident,
    generics: syn::Generics,
    data: Data<EnumVariants, ()>,
}

#[derive(Debug, FromVariant)]
struct EnumVariants {
    ident: syn::Ident,
    fields: Fields<EnumVariantField>,
}

#[derive(Debug, FromField)]
struct EnumVariantField {
    ty: syn::Type,
}
pub fn process_enum_from_darling(input: DeriveInput) -> TokenStream {
    let EnumFromDarling {
        ident,
        generics,
        data: Data::Enum(data),
    } = EnumFromDarling::from_derive_input(&input).unwrap()
    else {
        panic!("EnumFromDarling only works on enums")
    };
    let from_impls = data.iter().map(|variant| match variant.fields.style {
        Style::Tuple if variant.fields.len() == 1 => {
            let field = variant.fields.fields.first().expect("should have 1 field");
            let ty = &field.ty;
            let var = &variant.ident;
            quote! {
                impl #generics From<#ty> for #ident #generics {
                    fn from(v: #ty) -> Self {
                        #ident::#var(v)
                    }
                }
            }
        }
        _ => quote! {},
    });
    quote! {
        #(#from_impls)*
    }
}
