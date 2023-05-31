//! Easiest way to create a headless OpenGL context.
//!
//! Simply add <code>#[[gl_headless]]</code> to any function (even `main`),
//! then call that function as you otherwise would. Within the
//! scope of the function, an OpenGL context will be available.
//!
//! See all available options in the documentation for
//! <code>#[[gl_headless]]</code>.
//!
//! ## Simple Example
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
//! #[gl_headless]
//! unsafe fn main() {
//!     let (mut major, mut minor) = (0, 0);
//!     gl::GetIntegerv(gl::MAJOR_VERSION, &mut major);
//!     gl::GetIntegerv(gl::MINOR_VERSION, &mut minor);
//!     println!("OpenGL {major}.{minor}");
//! }
//! ```
//!
//! ## Specify OpenGL Version
//!
//! By default `#[gl_headless]` attempts to create an OpenGL 4.6 context. To use a specific version add, e.g. `version = "3.3"`:
//!
//! ```rust
//! use gl_headless::gl_headless;
//!
//! #[gl_headless(version = "3.3")]
//! unsafe fn main() {
//!     let (mut major, mut minor) = (3, 3);
//!     gl::GetIntegerv(gl::MAJOR_VERSION, &mut major);
//!     gl::GetIntegerv(gl::MINOR_VERSION, &mut minor);
//!     println!("OpenGL {major}.{minor}");
//! }
//! ```
//!
//! ## Multiple Functions
//!
//! Multiple functions can use `#[gl_headless]`:
//!
//! ```rust
//! use gl_headless::gl_headless;
//!
//! fn main() {
//!     unsafe {
//!         example1();
//!         example2();
//!     }
//! }
//!
//! #[gl_headless(version = "3.3")]
//! unsafe fn example1() {
//!     let (mut major, mut minor) = (3, 3);
//!     gl::GetIntegerv(gl::MAJOR_VERSION, &mut major);
//!     gl::GetIntegerv(gl::MINOR_VERSION, &mut minor);
//!     println!("OpenGL {major}.{minor}");
//! }
//!
//! #[gl_headless]
//! unsafe fn example2() {
//!     let mut handle = 0;
//!     gl::CreateBuffers(1, &mut handle);
//!
//!     let data: [f32; 5] = [1.0, 2.0, 3.0, 4.0, 5.0];
//!     gl::NamedBufferData(
//!         handle,
//!         std::mem::size_of_val(&data) as _,
//!         data.as_ptr() as *const _,
//!         gl::STATIC_DRAW,
//!     );
//!
//!     let mut byte_size = 0;
//!     gl::GetNamedBufferParameteriv(handle, gl::BUFFER_SIZE, &mut byte_size);
//!     let float_count = (byte_size as usize) / std::mem::size_of::<f32>() as usize;
//!
//!     let mut floats = vec![0.0_f32; float_count];
//!     gl::GetNamedBufferSubData(
//!         handle,
//!         0,
//!         std::mem::size_of_val(floats.as_slice()) as _,
//!         floats.as_mut_ptr() as *mut _,
//!     );
//!
//!     println!("Write: {:?}", data);
//!     println!("Read:  {:?}", floats);
//!
//!     assert_eq!(data, floats.as_slice());
//! }
//! ```
//!
//! [gl_headless]: https://docs.rs/gl-headless/*/gl_headless/attr.gl_headless.html

#![forbid(unsafe_code)]
#![forbid(elided_lifetimes_in_paths)]

#[doc(hidden)]
#[path = "internals.rs"]
pub mod _internals;

pub use gl_headless_macros::gl_headless;

use std::error;
use std::fmt;
use std::num::ParseIntError;

use glfw::InitError;

#[doc(hidden)]
#[derive(Debug)]
pub enum HeadlessError {
    Uninitialized(InitError),
    InvalidVersionFormat(GLVersionError),
    NoWindow,
}

impl error::Error for HeadlessError {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        match self {
            Self::Uninitialized(err) => Some(err),
            Self::InvalidVersionFormat(err) => Some(err),
            Self::NoWindow => None,
        }
    }
}

impl fmt::Display for HeadlessError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Uninitialized(err) => write!(f, "unable to initialize glfw: {err}"),
            Self::InvalidVersionFormat(err) => err.fmt(f),
            Self::NoWindow => write!(f, "no window"),
        }
    }
}

impl From<GLVersionError> for HeadlessError {
    #[inline]
    fn from(err: GLVersionError) -> Self {
        Self::InvalidVersionFormat(err)
    }
}

#[doc(hidden)]
#[derive(Debug)]
pub enum GLVersionError {
    InvalidVersion(String),
    InvalidMajor(String, ParseIntError),
    InvalidMinor(String, ParseIntError),
}

impl error::Error for GLVersionError {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        match self {
            Self::InvalidVersion(_) => None,
            Self::InvalidMajor(_, err) => Some(err),
            Self::InvalidMinor(_, err) => Some(err),
        }
    }
}

impl fmt::Display for GLVersionError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidVersion(version) => {
                write!(f, "invalid version format: {version:?}")
            }
            Self::InvalidMajor(major, err) => write!(f, "invalid major format {major:?}: {err}"),
            Self::InvalidMinor(minor, err) => write!(f, "invalid minor format {minor:?}: {err}"),
        }
    }
}
