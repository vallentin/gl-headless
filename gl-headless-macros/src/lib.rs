//! See [`gl-headless` docs] instead.
//!
//! [`gl-headless` docs]: https://docs.rs/gl-headless/*/gl_headless/

#![forbid(unsafe_code)]
#![forbid(elided_lifetimes_in_paths)]

use proc_macro::TokenStream;
use quote::quote_spanned;
use syn::spanned::Spanned;
use syn::{parse_macro_input, ItemFn};

/// Creates a headless OpenGL context.
///
/// See [crate root] root for examples.
///
/// [crate root]: https://docs.rs/gl-headless/*/gl_headless/
#[proc_macro_attribute]
pub fn gl_headless(args: TokenStream, item: TokenStream) -> TokenStream {
    let args_parser = syn::meta::parser(|meta| Err(meta.error("unsupported attribute")));
    parse_macro_input!(args with args_parser);

    let item_fn: ItemFn = parse_macro_input!(item);
    let attrs = &item_fn.attrs;
    let vis = &item_fn.vis;
    let ident = &item_fn.sig.ident;
    let sig = &item_fn.sig;

    quote_spanned! { item_fn.sig.span() =>
        #(#attrs)*
        #vis #sig {
            use ::gl_headless::_internals::prelude::*;
            let _ctx = GLContext::new().unwrap();
            #item_fn
            #ident()
        }
    }
    .into()
}
