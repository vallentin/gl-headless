# gl-headless

[![Latest Version](https://img.shields.io/crates/v/gl-headless.svg)](https://crates.io/crates/gl-headless)
[![Docs](https://docs.rs/gl-headless/badge.svg)](https://docs.rs/gl-headless)
[![License](https://img.shields.io/github/license/vallentin/gl-headless.svg)](https://github.com/vallentin/gl-headless)

Simplest way to create a headless OpenGL context.

## Dependencies

```toml
[dependencies]
gl = "0.14"
gl-headless = "0.2"
```

## Example

```rust
use gl_headless::gl_headless;

fn main() {
    unsafe {
        example();
    }
}

#[gl_headless]
unsafe fn example() {
    let mut buffers = [0_u32; 2];
    gl::CreateBuffers(buffers.len() as i32, buffers.as_mut_ptr());
    println!("{:?}", buffers);
}
```
