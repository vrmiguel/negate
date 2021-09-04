use proc_macro::TokenStream;
use proc_macro2::Span;
use quote::quote_spanned;

pub enum Error {
    InvalidIdentifier,
    /// A string literal was expected but not found
    StringLiteralExpected,
    /// Too many arguments were supplied, e.g. `name = negated_function, docs = "It's a negated function.`
    TooManyArgsSupplied,
    /// A nested structured meta item was expected but not found
    NestedMetaExpected,
    /// A name-value pair was expected but not found
    NameValueExpected,
    /// Expected names (in name-value pairs) are either "docs" or "name", but a different name was found
    UnexpectedName,
    ConflictingArgs,
}

pub fn build_compile_error(span: Span, err: Error) -> TokenStream {
    let err_tokens = match err {
        Error::StringLiteralExpected => quote_spanned! {
            span =>
            compile_error!("A string literal was expected but not found.");
        },
        Error::TooManyArgsSupplied => quote_spanned! {
            span =>
            compile_error!("Too many arguments were supplied.");
        },
        Error::NestedMetaExpected => quote_spanned! {
            span =>
            compile_error!("A nested structured meta item was expected but not found.");
        },
        Error::NameValueExpected => quote_spanned! {
            span =>
            compile_error!("A name-value pair was expected but not found");
        },
        Error::UnexpectedName => quote_spanned! {
            span =>
            compile_error!("Expected names (in name-value pairs) are either "docs" or "name", but a different name was found.");
        },
        Error::ConflictingArgs => quote_spanned! {
            span =>
            compile_error!("Conflicting arguments were found. E.g. `name = generated_fn, name = my_name`");
        },
        Error::InvalidIdentifier => quote_spanned! {
            span =>
            compile_error!("This identifier is invalid!")
        },
    };

    err_tokens.into()
}
