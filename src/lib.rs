mod args;
mod error;
mod generator;

use proc_macro::TokenStream;
use quote::quote_spanned;
use syn::{parse_macro_input, spanned::Spanned, AttributeArgs, Item};

use crate::args::parse_args;
use crate::generator::gen_negated_function;

/// The [`negate`](crate::negate) attribute macro creates a new function that _negates_ the function it was given.
/// For example, if applied to a function titled `is_student`, the macro will create a
/// new function named `is_not_student`, which simply negates the previous function.
///
/// ```rust
/// use negate::negate;
///
/// #[negate] // <- negate will implement a `is_not_even` function!
/// pub fn is_even(x: i32) -> bool {
///     x % 2 == 0
/// }
///
/// // We generated `is_not_even`!
/// assert!(is_not_even(5));
///
/// struct Word(&'static str);
///
/// impl Word {
///     pub fn new(word: &'static str) -> Self {
///         Self (word)
///     }
///
///     #[negate] // <- negate will implement a `is_not_uppercase` function!
///     pub fn is_uppercase(&self) -> bool {
///         self.0 == self.0.to_uppercase()
///     }
/// }
/// let my_name = Word::new("My Name");
///
/// // We generated `is_not_uppercase`!
/// assert!(my_name.is_not_uppercase());
/// ```
#[proc_macro_attribute]
pub fn negate(attrib_args: TokenStream, input: TokenStream) -> TokenStream {
    let args = parse_macro_input!(attrib_args as AttributeArgs);

    let item: syn::Item = syn::parse(input).expect("failed to parse input into `syn::Item`");

    let span = item.span();

    let args = match parse_args(args) {
        Ok(parsed_args) => parsed_args,
        Err(err) => return crate::error::build_compile_error(span, err)
    };

    match item {
        Item::Fn(function) => gen_negated_function(function, args),
        other => {
            let err = quote_spanned! {
                other.span() =>
                compile_error!("`negate` can only be applied to functions.");
            };

            err.into()
        }
    }
}
