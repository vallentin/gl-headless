pub mod prelude {
    pub use super::{GLContext, GLContextBuilder};
}

use std::str::FromStr;

use glfw::{Callback, Context, Glfw, OpenGlProfileHint, Window, WindowHint, WindowMode};

use crate::{GLVersionError, HeadlessError};

pub struct GLContextBuilder {
    version: Option<&'static str>,
}

impl GLContextBuilder {
    pub fn new() -> Self {
        Self { version: None }
    }

    pub fn set_version_str(&mut self, version: &'static str) {
        self.version = Some(version);
    }

    pub fn build(self) -> Result<GLContext, HeadlessError> {
        match self.version {
            Some(version) => GLContext::with_version_str(version),
            None => GLContext::new(),
        }
    }
}

pub struct GLContext {
    wnd: Window,
    glfw: Glfw,
}

impl GLContext {
    pub fn new() -> Result<Self, HeadlessError> {
        Self::with_version(GLVersion::default())
    }

    pub fn with_version_str(version: &str) -> Result<Self, HeadlessError> {
        Self::with_version(version.parse()?)
    }

    pub fn with_version(version: GLVersion) -> Result<Self, HeadlessError> {
        let mut glfw = glfw::init(Some(Callback {
            f: |err, desc, _| panic!("glfw error [{}]: {}", err, desc),
            data: (),
        }))
        .map_err(HeadlessError::Uninitialized)?;

        let GLVersion(major, minor) = version;
        glfw.window_hint(WindowHint::ContextVersion(major, minor));
        glfw.window_hint(WindowHint::OpenGlProfile(OpenGlProfileHint::Core));
        glfw.window_hint(WindowHint::OpenGlForwardCompat(true));
        glfw.window_hint(WindowHint::Visible(false));

        let (mut wnd, _events) = glfw
            .create_window(1280, 720, env!("CARGO_PKG_NAME"), WindowMode::Windowed)
            .ok_or_else(|| HeadlessError::NoWindow)?;

        wnd.make_current();

        gl::load_with(|symbol| wnd.get_proc_address(symbol) as *const _);

        Ok(Self { wnd, glfw })
    }

    #[inline]
    pub fn glfw_mut(&mut self) -> &mut Glfw {
        &mut self.glfw
    }

    #[inline]
    pub fn window_mut(&mut self) -> &mut Window {
        &mut self.wnd
    }
}

#[derive(Debug)]
pub struct GLVersion(pub u32, pub u32);

impl Default for GLVersion {
    fn default() -> Self {
        Self(4, 6)
    }
}

impl FromStr for GLVersion {
    type Err = GLVersionError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (major, minor) = s
            .split_once('.')
            .ok_or_else(|| GLVersionError::InvalidVersion(s.to_owned()))?;
        let major = major
            .parse()
            .map_err(|err| GLVersionError::InvalidMajor(major.to_owned(), err))?;
        let minor = minor
            .parse()
            .map_err(|err| GLVersionError::InvalidMinor(minor.to_owned(), err))?;
        Ok(Self(major, minor))
    }
}
