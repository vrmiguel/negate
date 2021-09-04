//! [![github]](https://github.com/vrmiguel/negate)&ensp;[![crates-io]](https://crates.io/crates/negate)&ensp;[![docs-rs]](crate)
//!
//! [github]: https://img.shields.io/badge/github-8da0cb?style=for-the-badge&labelColor=555555&logo=github
//! [crates-io]: https://img.shields.io/badge/crates.io-fc8d62?style=for-the-badge&labelColor=555555&logo=rust
//! [docs-rs]: https://img.shields.io/badge/docs.rs-66c2a5?style=for-the-badge&labelColor=555555&logoColor=white&logo=data:image/svg+xml;base64,PHN2ZyByb2xlPSJpbWciIHhtbG5zPSJodHRwOi8vd3d3LnczLm9yZy8yMDAwL3N2ZyIgdmlld0JveD0iMCAwIDUxMiA1MTIiPjxwYXRoIGZpbGw9IiNmNWY1ZjUiIGQ9Ik00ODguNiAyNTAuMkwzOTIgMjE0VjEwNS41YzAtMTUtOS4zLTI4LjQtMjMuNC0zMy43bC0xMDAtMzcuNWMtOC4xLTMuMS0xNy4xLTMuMS0yNS4zIDBsLTEwMCAzNy41Yy0xNC4xIDUuMy0yMy40IDE4LjctMjMuNCAzMy43VjIxNGwtOTYuNiAzNi4yQzkuMyAyNTUuNSAwIDI2OC45IDAgMjgzLjlWMzk0YzAgMTMuNiA3LjcgMjYuMSAxOS45IDMyLjJsMTAwIDUwYzEwLjEgNS4xIDIyLjEgNS4xIDMyLjIgMGwxMDMuOS01MiAxMDMuOSA1MmMxMC4xIDUuMSAyMi4xIDUuMSAzMi4yIDBsMTAwLTUwYzEyLjItNi4xIDE5LjktMTguNiAxOS45LTMyLjJWMjgzLjljMC0xNS05LjMtMjguNC0yMy40LTMzLjd6TTM1OCAyMTQuOGwtODUgMzEuOXYtNjguMmw4NS0zN3Y3My4zek0xNTQgMTA0LjFsMTAyLTM4LjIgMTAyIDM4LjJ2LjZsLTEwMiA0MS40LTEwMi00MS40di0uNnptODQgMjkxLjFsLTg1IDQyLjV2LTc5LjFsODUtMzguOHY3NS40em0wLTExMmwtMTAyIDQxLjQtMTAyLTQxLjR2LS42bDEwMi0zOC4yIDEwMiAzOC4ydi42em0yNDAgMTEybC04NSA0Mi41di03OS4xbDg1LTM4Ljh2NzUuNHptMC0xMTJsLTEwMiA0MS40LTEwMi00MS40di0uNmwxMDItMzguMiAxMDIgMzguMnYuNnoiPjwvcGF0aD48L3N2Zz4K
//!
//! <br>
//!
//! [`negate`](crate::negate) is a simple attribute macro that negates a given function.
//!
//! - `[negate]` <p>Given a function of the form `is_*` that returns a boolean value,
//!   the macro will create a `is_not_*` that negates the given function.</p>
//!   
//!   ```rust
//!   use negate::negate;
//!   #[negate]
//!   pub fn is_even(x: i32) -> bool {
//!     x % 2 == 0
//!   }
//!
//!   // The `is_not_even` function was generated!
//!   assert!(is_not_even(5));
//!   ```

//!   For reference, this is how the generated function looks like:
//!   ```ignore,rust
//!   /// This is an automatically generated function that denies [`is_even`].
//!   /// Consult the original function for more information.
//!   pub fn is_not_even(x: i32) -> bool {
//!     !is_even(x)
//!   }
//!   ```
//!
//! - `[negate(name = "...")]` <p>Using the `name` attribute allows you to set the name of the generated function.
//!   This also allows the usage of the [`negate`] macro with functions that do not start with `is_`.
//!
//! ```rust
//! use negate::negate;
//! use std::collections::HashMap;
//!
//! pub enum TaskState {
//!     Ready,
//!     Finished,
//! }
//!
//! pub struct Reactor {
//!     tasks: HashMap<usize, TaskState>,
//! }
//!
//! impl Reactor {
//!     // Generates the `is_finished` function
//!     #[negate(name = "is_finished")]
//!     pub fn is_ready(&self, id: usize) -> bool {
//!         self.tasks.get(&id).map(|state| match state {
//!             TaskState::Ready => true,
//!             _ => false,
//!         }).unwrap_or(false)
//!     }
//! }
//! ```
//! - `[negate(docs = "...")]` <p>Using the `docs` attribute allows you to customize
//!   the doc-string of the generated function</p>
//! ```rust
//! use negate::negate;
//! #[negate(name = "is_odd", docs = "returns true if the given number is odd")]
//! fn is_even(x: i32) -> bool {
//!    x % 2 == 0
//! }
//! assert!(is_odd(5));
//! ```

mod args;
mod error;
mod generator;

use proc_macro::TokenStream;
use syn::{parse_macro_input, spanned::Spanned, AttributeArgs, Item};

use crate::args::parse_args;
use crate::error::Error;
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
        Err(err) => return crate::error::build_compile_error(span, err),
    };

    match item {
        Item::Fn(function) => gen_negated_function(function, args),
        other => error::build_compile_error(other.span(), Error::NotAppliedToAFunction),
    }
}
