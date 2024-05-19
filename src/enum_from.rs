use proc_macro2::TokenStream;
use quote::quote;
use syn::{Data, DeriveInput};

pub fn process_enum_from(input: DeriveInput) -> TokenStream {
    let ident = input.ident;
    let generics = input.generics;
    let from_impls = match input.data {
        Data::Enum(enum_data) => {
            enum_data
                .variants
                .into_iter()
                .map(|variant| match variant.fields {
                    syn::Fields::Unnamed(unnamed) => {
                        let var = variant.ident;
                        let field = unnamed
                            .unnamed
                            .first()
                            .expect("EnumRrom only support 1 argument");
                        let ty = &field.ty;
                        quote! {
                            impl #generics From<#ty> for #ident #generics {
                                fn from(v: #ty) -> Self {
                                    #ident::#var(v)
                                }
                            }
                        }
                    }
                    _ => quote! {},
                })
        }
        _ => panic!("EnumFrom can only be used on Enum"),
    };
    quote! {
        #(#from_impls)*
    }
}
