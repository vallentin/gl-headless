//! Simplest way to create a headless OpenGL context.
//!
//! ```rust
//! use gl_headless::gl_headless;
//!
//! fn main() {
//!     unsafe {
//!         example();
//!     }
//! }
//!
//! #[gl_headless]
//! unsafe fn example() {
//!     let mut buffers = [0_u32; 2];
//!     gl::CreateBuffers(buffers.len() as i32, buffers.as_mut_ptr());
//!     println!("{:?}", buffers);
//! }
//! ```

#![forbid(elided_lifetimes_in_paths)]

use proc_macro::TokenStream;
use quote::quote_spanned;
use syn::spanned::Spanned;
use syn::{parse_macro_input, ItemFn};

/// Creates a headless OpenGL context.
///
/// See [crate root](crate) root for examples.
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
            use ::glfw::*;

            let mut glfw = glfw::init(Some(glfw::Callback {
                f: |err, desc, _| panic!("glfw error [{}]: {}", err, desc),
                data: (),
            }))
            .expect("unable to initialize glfw");

            glfw.window_hint(WindowHint::ContextVersion(4, 6));
            glfw.window_hint(WindowHint::OpenGlProfile(OpenGlProfileHint::Core));
            glfw.window_hint(WindowHint::OpenGlForwardCompat(true));
            glfw.window_hint(WindowHint::Visible(false));

            let (mut wnd, _events) = glfw
                .create_window(1280, 720, env!("CARGO_PKG_NAME"), WindowMode::Windowed)
                .unwrap();

            wnd.make_current();

            gl::load_with(|symbol| wnd.get_proc_address(symbol) as *const _);

            #item_fn
            #ident()
        }
    }
    .into()
}
