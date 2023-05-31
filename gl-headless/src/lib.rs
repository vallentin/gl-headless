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
