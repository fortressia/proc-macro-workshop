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
    let mut setters = Vec::new();
    let mut builder_setters = Vec::new();
    let mut checkers = Vec::new();
    match input.data {
        Data::Struct(data) => match data.fields {
            syn::Fields::Named(fields) => {
                for field in fields.named.into_iter() {
                    let field_ident = field.ident.unwrap();

                    let the_type = field.ty;
                    properties.push(quote!(
                        #field_ident: Option<#the_type>,
                    ));


                    setters.push(quote!(
                        pub fn #field_ident(&mut self, #field_ident: #the_type) -> &mut Self {
                            self.#field_ident = Some(#field_ident);

                            self
                        }
                    ));
                
                    builder_setters.push(quote!(
                        #field_ident: self.#field_ident.clone().unwrap(),
                    ));

                    checkers.push(quote!(
                        if self.#field_ident.is_none() {
                       
                           return Err(Box::<dyn Error>::from("Property #field_ident is missing. Assign it by calling #builder_ident::#field_ident()"));
                        }
                    ))
                }
            }

            _ => unimplemented!(),
        },
        _ => unimplemented!(),
    };

    let output = quote!(
        #[derive(Clone, Default)]
        pub struct #builder_ident {
            #(#properties)*
        }

        impl #builder_ident {
            #(#setters)*

            pub fn new() -> Self {
                Self::default()
            }

            pub fn build(&mut self) -> Result<#ident, Box<dyn std::error::Error>> {
                use std::error::Error;
          
                #(#checkers)*

                Ok(#ident {
                    #(#builder_setters)*
                })
            }
        }

        impl #ident {
            pub fn builder() -> #builder_ident {
                #builder_ident::new()
            }
        }

    );

  

    output.into()
   
}
