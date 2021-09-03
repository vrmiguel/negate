use std::mem;

use proc_macro::TokenStream;
use proc_macro2::Span;
use quote::{quote, ToTokens};
use syn::{FnArg, Ident, ItemFn, Pat, ReturnType, Type};

pub fn gen_negated_function(func: ItemFn) -> TokenStream {
    let negated_identifier = {
        let signature = &func.sig;

        // The particular type this function returns
        let output_type = &signature.output;

        // Make sure this function returns bool
        //
        // We're not currently able to type resolve so aliases or
        // new-types around bool will fail this check :c
        assert!(returns_bool(output_type));

        negate_identifier(&signature.ident)
    };

    // We must replicate the original function
    let original_function = func.clone();

    let mut signature = func.sig;
    let visibility = func.vis;

    let original_identifier = mem::replace(
        &mut signature.ident,
        Ident::new(&negated_identifier, Span::call_site()),
    );

    let pattern_from_arg = |arg: &FnArg| -> Pat {
        match arg {
            FnArg::Receiver(_) => unimplemented!("I don't know what to do about this yet"),
            FnArg::Typed(pat_type) => (&*pat_type.pat).clone(),
        }
    };

    let arguments = signature.inputs.iter().map(pattern_from_arg);

    let tokens = quote! {

        #original_function

        #visibility #signature {
            !(#original_identifier(#(#arguments),*) )
        }
    };

    tokens.into()
}

fn returns_bool(return_type: &ReturnType) -> bool {
    fn type_is_bool(ty: &Type) -> bool {
        matches!(ty, Type::Path(type_path) if type_path.to_token_stream().to_string() == "bool")
    }

    match return_type {
        ReturnType::Default => false,
        ReturnType::Type(_arrow, ty) => type_is_bool(ty),
    }
}

fn negate_identifier(ident: &Ident) -> String {
    let identifier = ident.to_string();

    assert!(identifier.starts_with("is_"));

    // Assuming we get an input of the form `is_adjective` (e.g. `is_even`, `is_odd`),
    // we want to extract just the adjective in order to negate it right after.
    let adjective = identifier.get(3..).expect("The identifier is too short!");

    format!("is_not_{}", adjective)
}
