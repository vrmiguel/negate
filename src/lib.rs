mod generator;

use proc_macro::TokenStream;
use syn::Item;

use crate::generator::gen_negated_function;

#[proc_macro_attribute]
pub fn negate(_metadata: TokenStream, input: TokenStream) -> TokenStream {
    let item: syn::Item = syn::parse(input).expect("failed to parse input into `syn::Item`");

    match item {
        Item::Fn(function) => gen_negated_function(function),
        _other => {
            unreachable!("proper compilation error here");
            // quote! { #other }
        }
    }
}
