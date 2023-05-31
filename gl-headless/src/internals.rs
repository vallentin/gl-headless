pub mod prelude {
    pub use super::GLContext;
}

use glfw::{Callback, Context, Glfw, OpenGlProfileHint, Window, WindowHint, WindowMode};

use crate::HeadlessError;

pub struct GLContext {
    wnd: Window,
    glfw: Glfw,
}

impl GLContext {
    pub fn new() -> Result<Self, HeadlessError> {
        let mut glfw = glfw::init(Some(Callback {
            f: |err, desc, _| panic!("glfw error [{}]: {}", err, desc),
            data: (),
        }))
        .map_err(HeadlessError::Uninitialized)?;

        glfw.window_hint(WindowHint::ContextVersion(4, 6));
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
