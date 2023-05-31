//! Simplest way to create a headless OpenGL context.
//!
//! Dependencies:
//!
//! ```toml
//! [dependencies]
//! gl = "0.14"
//! gl-headless = "0.2"
//! ```
//!
//! Example:
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

#![forbid(unsafe_code)]
#![forbid(elided_lifetimes_in_paths)]

#[doc(hidden)]
#[path = "internals.rs"]
pub mod _internals;

pub use gl_headless_macros::gl_headless;

use std::error;
use std::fmt;

use glfw::InitError;

#[doc(hidden)]
#[derive(Debug)]
pub enum HeadlessError {
    Uninitialized(InitError),
    NoWindow,
}

impl error::Error for HeadlessError {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        match self {
            Self::Uninitialized(err) => Some(err),
            Self::NoWindow => None,
        }
    }
}

impl fmt::Display for HeadlessError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Uninitialized(err) => write!(f, "unable to initialize glfw: {err}"),
            Self::NoWindow => write!(f, "no window"),
        }
    }
}
