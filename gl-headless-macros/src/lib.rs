//! See [`gl-headless` docs] instead.
//!
//! [`gl-headless` docs]: https://docs.rs/gl-headless/*/gl_headless/

#![forbid(unsafe_code)]
#![forbid(elided_lifetimes_in_paths)]

use proc_macro::TokenStream;
use quote::quote_spanned;
use syn::spanned::Spanned;
use syn::{parse_macro_input, ItemFn, LitStr};

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

    let call_wrap_unsafe = sig.unsafety.is_some() && (ident.to_string() == "main");
    let call = if call_wrap_unsafe {
        new_sig.unsafety = None;

        quote_spanned! { sig.span() =>
            unsafe {
                #ident()
            }
        }
    } else {
        quote_spanned! { sig.span() =>
            #ident()
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
            let _ctx = {
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
