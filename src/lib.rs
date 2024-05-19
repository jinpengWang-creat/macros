// proc macro crate

// for enum, we'd like to generate From impls for each variant
use proc_macro::TokenStream;
use quote::quote;
use syn::Data;

#[proc_macro_derive(EnumFrom)]
pub fn derive_enum_from(input: TokenStream) -> TokenStream {
    let input = syn::parse_macro_input!(input as syn::DeriveInput);

    let ident = &input.ident;

    let from_impls = match &input.data {
        Data::Enum(enum_data) => enum_data
            .variants
            .iter()
            .map(|variant| match &variant.fields {
                syn::Fields::Unnamed(unnamed) => {
                    let var = &variant.ident;
                    let field = unnamed
                        .unnamed
                        .first()
                        .expect("EnumRrom only support 1 argument");
                    let ty = &field.ty;
                    quote! {
                        impl From<#ty> for #ident {
                            fn from(v: #ty) -> Self {
                                #ident::#var(v)
                            }
                        }
                    }
                }
                _ => quote! {},
            }),
        _ => panic!("EnumFrom can only be used on Enum"),
    };
    quote! {
        #(#from_impls)*
    }
    .into()
}
