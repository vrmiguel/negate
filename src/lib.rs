mod generator;

use proc_macro::TokenStream;
use quote::quote_spanned;
use syn::{parse_macro_input, spanned::Spanned, AttributeArgs, Item, Lit, NestedMeta};

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
pub fn negate(args: TokenStream, input: TokenStream) -> TokenStream {
    let args = parse_macro_input!(args as AttributeArgs);

    let item: syn::Item = syn::parse(input).expect("failed to parse input into `syn::Item`");

    let maybe_docs = match parse_arg(args) {
        Ok(maybe_docs) => maybe_docs,
        Err(_) => {
            let err = quote_spanned! {
                item.span() => compile_error!("input to `negate` must be a name-value pair, such as `docs = \"my documentation\"`");
            };
            return err.into();
        }
    };

    match item {
        Item::Fn(function) => gen_negated_function(function, maybe_docs),
        other => {
            let err = quote_spanned! {
                other.span() =>
                compile_error!("`negate` can only be applied to functions.");
            };

            err.into()
        }
    }
}

/// The user-supplied docs for a negated function
type MaybeDocs = Option<String>;

fn parse_arg(args: AttributeArgs) -> Result<MaybeDocs, ()> {
    match args.len() {
        // User didn't supply any docs to the generated functions, so
        // we'll use our default documentation.
        0 => return Ok(None),
        1 => {}
        _ => panic!("too many arguments supplied to macro"), // _ => return Err("too many arguments supplied to macro")
    }

    let parse_arg_into_str = |arg: &NestedMeta| -> Option<_> {
        let meta = match arg {
            NestedMeta::Meta(meta) => meta,
            NestedMeta::Lit(_) => None?,
        };

        let pair = match meta {
            syn::Meta::NameValue(pair) => pair,
            _ => None?,
        };

        let lit = if !pair.path.is_ident("docs") {
            None?
        } else {
            &pair.lit
        };

        if let Lit::Str(lit_str) = lit {
            Some(lit_str.value())
        } else {
            None
        }
    };

    let user_supplied_docs = parse_arg_into_str(&args[0]).ok_or(())?;

    Ok(Some(user_supplied_docs))
}
