//! Simplest way to create a headless OpenGL context.
//!
//! ```toml
//! [dependencies]
//! gl = "0.14"
//! gl-headless = "0.2"
//! ```
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

pub use gl_headless_macros::gl_headless;

#[doc(hidden)]
pub use glfw;
