use proc_macro::TokenStream;
use quote::{format_ident, quote};
use syn::{parse, Data, DeriveInput};

#[proc_macro_derive(Builder)]
pub fn derive(input: TokenStream) -> TokenStream {
    let input: DeriveInput = parse(input).unwrap();
    let ident = input.ident;
    // let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();
    let builder_ident = format_ident!("{}Builder", &ident);
    let mut properties = Vec::new();
    let mut builder_init = Vec::new();
    match input.data {
        Data::Struct(data) => match data.fields {
            syn::Fields::Named(fields) => {
                for field in fields.named.into_iter() {
                    let ident = field.ident.unwrap();

                    let the_type = field.ty;
                    properties.push(quote!(
                        #ident: Option<#the_type>,
                    ));

                    builder_init.push(quote!(
                        #ident: None,
                    ));
                }
            }

            syn::Fields::Unnamed(_) | syn::Fields::Unit => unimplemented!(),
        },
        _ => unimplemented!(),
    };

    quote!(
        pub struct #builder_ident {
         #(#properties)*
        }

        impl #ident {
            pub fn builder() -> #builder_ident {
                #builder_ident {
                  #(#builder_init)*
                }
            }
        }

    )
    .into()
}
