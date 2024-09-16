
#![crate_type = "proc-macro"]
extern crate proc_macro;
use proc_macro::TokenStream;
use syn::{braced, parse_macro_input, token, Attribute, Field, Ident, Item, Result, Token};
use syn::parse::{Parse, ParseStream};
use syn::punctuated::Punctuated;
use quote::quote;


#[proc_macro_attribute]
pub fn pho_mapped(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(item as Item);

    if let Item::Struct(s) = ast {
        let name = s.ident;
        let fields = s.fields.iter();
        let attrs = s.attrs.into_iter().map(|a| {
            a.pound_token
        });
        quote! {
            #(#attrs)*
            pub struct #name { #(#fields),* }
        }.into()
    } else {
        quote! {}.into()
    }
}