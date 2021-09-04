use proc_macro::TokenStream;
use proc_macro2::Span;
use quote::{quote, ToTokens};
use syn::{spanned::Spanned, FnArg, Ident, ItemFn, Pat, ReturnType, Signature, Type};

use crate::{
    args::Args,
    error::{self, Error},
};

/// Extracts a type ascription pattern from a function argument
///
/// e.g.: if the argument is `name: &str`, returns the pattern that represents `name`.
///
/// Assumes that this argument isn't a receiver (e.g. `self` or `&self` or `&mut self`)
fn pattern_from_arg(arg: &FnArg) -> &Pat {
    match arg {
        FnArg::Receiver(_) => unreachable!("`pattern_from_arg` should never receive patterns"),
        FnArg::Typed(pat_type) => &*pat_type.pat,
    }
}

/// Returns true if this signature represents an associated function
fn is_associated_function(signature: &Signature) -> bool {
    fn is_receiver(arg: &FnArg) -> bool {
        matches!(arg, FnArg::Receiver(_))
    }

    signature.inputs.iter().any(is_receiver)
}

pub fn gen_negated_function(func: ItemFn, args: Args) -> TokenStream {
    let maybe_name = args.name;
    let maybe_docs = args.docs;

    let negated_identifier = {
        let signature = &func.sig;

        // The particular type this function returns
        let output_type = &signature.output;

        // Make sure this function returns bool
        //
        // We're not currently able to type resolve so aliases or
        // new-types around bool will fail this check :c
        if !returns_bool(output_type) {
            return error::build_compile_error(func.span(), Error::DoesNotReturnBool);
        }

        match build_identifier(maybe_name, &func) {
            Ok(id) => id,
            Err(err) => {
                return error::build_compile_error(func.span(), err);
            }
        }
    };

    // The proc-macro attribute "consumes" the implementation of the function being negated,
    // so we must must replicate the original function.
    let original_function = func;

    // Will represent the signature of our negated function!
    let mut new_signature = original_function.sig.clone();

    new_signature.ident = Ident::new(&negated_identifier, Span::call_site());

    let doc_string = build_docstring(maybe_docs, &original_function);

    if is_associated_function(&new_signature) {
        generate_associated_fn(original_function, new_signature, doc_string)
    } else {
        generate_non_associated_fn(original_function, new_signature, doc_string)
    }
}

/// Generates a negated function, where this function is
/// not associated.
///
/// # Examples
/// ```rust
/// use negate::negate;
///
/// struct Word(&'static str);
///
/// impl Word {
///     pub fn new(word: &'static str) -> Self {
///         Self (word)
///     }
///
///     #[negate]
///     pub fn is_uppercase(&self) -> bool {
///         self.0 == self.0.to_uppercase()
///     }
/// }
/// let my_name = Word::new("My Name");
/// assert!(my_name.is_not_uppercase());
/// ```
fn generate_associated_fn(
    original_function: ItemFn,
    new_signature: Signature,
    doc_string: String,
) -> TokenStream {
    let visibility = &original_function.vis;
    let arguments = new_signature.inputs.iter().skip(1).map(pattern_from_arg);
    let original_identifier = &original_function.sig.ident;

    let tokens = quote! {
        #original_function

        #[doc = #doc_string]
        #visibility #new_signature {
            !self.#original_identifier(#(#arguments),*)
        }
    };

    tokens.into()
}

/// Generates a negated function, where this function is
/// not associated.
///
/// # Examples
/// ```rust
/// use negate::negate;
/// #[negate]
/// fn is_even(x: i32) -> bool {
///     x % 2 == 0
/// }
/// // `is_not_even` was generated
/// assert!(is_not_even(3));
/// ```
fn generate_non_associated_fn(
    original_function: ItemFn,
    new_signature: Signature,
    doc_string: String,
) -> TokenStream {
    let visibility = &original_function.vis;
    let arguments = new_signature.inputs.iter().map(pattern_from_arg);
    let original_identifier = &original_function.sig.ident;

    let tokens = quote! {
        #original_function

        #[doc = #doc_string]
        #visibility #new_signature {
            !#original_identifier(#(#arguments),*)
        }
    };

    tokens.into()
}

fn build_docstring(maybe_docs: Option<String>, original_function: &ItemFn) -> String {
    let original_identifier = &original_function.sig.ident;

    let gen_docs = || {
        let ident = original_identifier.to_string();
        format!("This is an automatically generated function that denies [`{}`].\nConsult the original function for more information.", ident)
    };

    maybe_docs.unwrap_or_else(gen_docs)
}

/// If the user supplied a name for the generated function, this function will return it.
/// If the user didn't, then we'll attempt to generate a name for the generated function
fn build_identifier(
    maybe_name: Option<String>,
    original_function: &ItemFn,
) -> Result<String, Error> {
    if let Some(name) = maybe_name {
        Ok(name)
    } else {
        let original_identifier = &original_function.sig.ident;
        negate_identifier(original_identifier)
    }
}

/// Returns true if the given return type is a boolean value.
///
/// Attribute macros are not able to type resolve (at least at of writing) so
/// aliases or new-types around bool will fail this check.
fn returns_bool(return_type: &ReturnType) -> bool {
    fn type_is_bool(ty: &Type) -> bool {
        matches!(ty, Type::Path(type_path) if type_path.to_token_stream().to_string() == "bool")
    }

    match return_type {
        ReturnType::Default => false,
        ReturnType::Type(_arrow, ty) => type_is_bool(ty),
    }
}

fn get_adjective(identifier: &str) -> Option<&str> {
    if !identifier.starts_with("is_") {
        None?;
    }

    // Assuming we get an input of the form `is_adjective` (e.g. `is_even`, `is_odd`),
    // we want to extract just the adjective in order to negate it right after.
    identifier.get(3..)
}

fn negate_identifier(ident: &Ident) -> Result<String, Error> {
    let identifier = ident.to_string();

    let adjective = get_adjective(&identifier).ok_or(Error::InvalidIdentifier)?;

    Ok(format!("is_not_{}", adjective))
}
