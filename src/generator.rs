use proc_macro::TokenStream;
use proc_macro2::Span;
use quote::{quote, quote_spanned, ToTokens};
use syn::{spanned::Spanned, FnArg, Ident, ItemFn, Pat, ReturnType, Signature, Type};

/// Extracts a type ascription pattern from a function argument
///
/// e.g.: if the argument is `name: &str`, returns the pattern that represents `name`.
///
/// Assumes that this argument isn't a receiver (e.g. `self` or `&self` or `&mut self`)
fn pattern_from_arg(arg: &FnArg) -> &Pat {
    match arg {
        FnArg::Receiver(_) => unimplemented!("I don't know what to do about this yet"),
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

pub fn gen_negated_function(func: ItemFn) -> TokenStream {
    let negated_identifier = {
        let signature = &func.sig;

        // The particular type this function returns
        let output_type = &signature.output;

        // Make sure this function returns bool
        //
        // We're not currently able to type resolve so aliases or
        // new-types around bool will fail this check :c
        if !returns_bool(output_type) {
            let err = quote_spanned! {
                func.span() =>
                compile_error!("the function does not seem to return a boolean value.");
            };

            return err.into();
        }

        match negate_identifier(&signature.ident) {
            Some(id) => id,
            None => {
                let err = quote_spanned! {
                    func.span() =>
                    compile_error!("`negate` can only be applied to functions.");
                };

                return err.into();
            }
        }
    };

    // The proc-macro attribute "consumes" the implementation of the function being negated,
    // so we must must replicate the original function.
    let original_function = func;

    // Will represent the signature of our negated function!
    let mut new_signature = original_function.sig.clone();

    new_signature.ident = Ident::new(&negated_identifier, Span::call_site());

    if is_associated_function(&new_signature) {
        todo!("not done yet, chief");
    } else {
        generate_non_associated_fn(original_function, new_signature)
    }
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
fn generate_non_associated_fn(original_function: ItemFn, new_signature: Signature) -> TokenStream {
    let visibility = &original_function.vis;    
    let arguments = new_signature.inputs.iter().map(pattern_from_arg);
    let original_identifier = &original_function.sig.ident;

    let tokens = quote! {
        #original_function

        #visibility #new_signature {
            !(#original_identifier(#(#arguments),*) )
        }
    };

    tokens.into()
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

fn negate_identifier(ident: &Ident) -> Option<String> {
    let identifier = ident.to_string();

    if !identifier.starts_with("is_") {
        return None;
    }

    // Assuming we get an input of the form `is_adjective` (e.g. `is_even`, `is_odd`),
    // we want to extract just the adjective in order to negate it right after.
    let adjective = identifier.get(3..).expect("The identifier is too short!");

    Some(format!("is_not_{}", adjective))
}
