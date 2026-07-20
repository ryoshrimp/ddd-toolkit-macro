use proc_macro_crate::{FoundCrate, crate_name};
use proc_macro2::{Span, TokenStream as TokenStream2};
use quote::quote;
use syn::Ident;

/// Resolves the path to `ddd-toolkit-core`'s public API as seen from the
/// crate that is invoking one of this crate's derive macros.
///
/// Consumers may depend on `ddd-toolkit-core` directly, or only
/// transitively through the `ddd-toolkit` facade crate (which re-exports
/// it). Try both, in that order, resolved fresh per macro invocation so a
/// build that mixes both usage patterns across different crates still gets
/// the right answer at each call site.
pub fn core_crate_path() -> TokenStream2 {
    for name in ["ddd-toolkit-core", "ddd-toolkit"] {
        if let Ok(found) = crate_name(name) {
            return match found {
                FoundCrate::Itself => quote!(crate),
                FoundCrate::Name(name) => {
                    let ident = Ident::new(&name, Span::call_site());
                    quote!(::#ident)
                }
            };
        }
    }
    // Neither name could be resolved (e.g. CARGO_MANIFEST_DIR unset, as can
    // happen with some non-Cargo build setups) - fall back to the historical
    // hardcoded path rather than hard-failing every expansion.
    quote!(::ddd_toolkit_core)
}
