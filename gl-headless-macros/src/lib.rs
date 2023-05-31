//! See [`gl-headless` docs] instead.
//!
//! [`gl-headless` docs]: https://docs.rs/gl-headless/*/gl_headless/

#![forbid(unsafe_code)]
#![forbid(elided_lifetimes_in_paths)]

use std::fmt;

use proc_macro::TokenStream;
use quote::{quote_spanned, ToTokens};
use syn::spanned::Spanned;
use syn::{parse_macro_input, FnArg, ItemFn, LitStr, Pat};

/// Creates a headless OpenGL context, that is valid throughout
/// the scope of the function.
///
/// See examples in the [crate root].
///
/// # Attributes
///
/// - `version = "3.3"`: Specify the OpenGL version, e.g.:
///   `#[gl_headless(version = "3.3")]`
///
/// # Example
///
/// ```toml
/// [dependencies]
/// gl = "0.14"
/// gl-headless = "0.2"
/// ```
///
/// ```rust
/// use gl_headless::gl_headless;
///
/// #[gl_headless]
/// unsafe fn main() {
///     let (mut major, mut minor) = (0, 0);
///     gl::GetIntegerv(gl::MAJOR_VERSION, &mut major);
///     gl::GetIntegerv(gl::MINOR_VERSION, &mut minor);
///     println!("OpenGL {major}.{minor}");
/// }
/// ```
///
/// [crate root]: https://docs.rs/gl-headless/*/gl_headless/
#[proc_macro_attribute]
pub fn gl_headless(args: TokenStream, item: TokenStream) -> TokenStream {
    let mut version: Option<LitStr> = None;
    let args_parser = syn::meta::parser(|meta| {
        if meta.path.is_ident("version") {
            version = Some(meta.value()?.parse()?);
            Ok(())
        } else {
            Err(meta.error("unsupported attribute"))
        }
    });
    parse_macro_input!(args with args_parser);

    let item_fn: ItemFn = parse_macro_input!(item);
    let attrs = &item_fn.attrs;
    let vis = &item_fn.vis;
    let sig = &item_fn.sig;
    let ident = &sig.ident;

    let mut new_sig = sig.clone();

    let mut args = Vec::with_capacity(sig.inputs.len());
    for arg in &sig.inputs {
        let arg = match arg {
            FnArg::Typed(arg) => arg,
            FnArg::Receiver(_arg) => {
                return error(arg, "associated methods not supported currently")
            }
        };
        let ident = match arg.pat.as_ref() {
            Pat::Ident(ident) => ident,
            _ => {
                return error(arg, "pattern not supported currently");
            }
        };
        args.push(&ident.ident);
    }

    let call_wrap_unsafe = sig.unsafety.is_some() && (ident.to_string() == "main");
    let call = if call_wrap_unsafe {
        new_sig.unsafety = None;

        quote_spanned! { sig.span() =>
            unsafe {
                #ident(
                    #(#args),*
                )
            }
        }
    } else {
        quote_spanned! { sig.span() =>
            #ident(
                #(#args),*
            )
        }
    };

    let set_version_str = version.map(|version| {
        quote_spanned! { version.span() =>
            builder.set_version_str(#version);
        }
    });

    quote_spanned! { sig.span() =>
        #(#attrs)*
        #vis #new_sig {
            let __gl_headless_ctx = {
                #[allow(unused_mut)]
                let mut builder = ::gl_headless::_internals::GLContextBuilder::new();
                #set_version_str
                builder.build().unwrap()
            };
            #item_fn
            #call
        }
    }
    .into()
}

#[must_use]
fn error<T: ToTokens, U: fmt::Display>(tokens: T, message: U) -> TokenStream {
    syn::Error::new_spanned(tokens, message)
        .into_compile_error()
        .into()
}
