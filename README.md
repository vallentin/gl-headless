# gl-headless

[![Latest Version](https://img.shields.io/crates/v/gl-headless.svg)](https://crates.io/crates/gl-headless)
[![Docs](https://docs.rs/gl-headless/badge.svg)](https://docs.rs/gl-headless)
[![License](https://img.shields.io/github/license/vallentin/gl-headless.svg)](https://github.com/vallentin/gl-headless)

Easiest way to create a headless OpenGL context.

Simply add <code>#[[gl_headless]]</code> to any function (even `main`), then call that function as you otherwise would. Within the scope of the function, an OpenGL context will be available.

See all available options in the documentation for
<code>#[[gl_headless]]</code>.

## Simple Example

```toml
[dependencies]
gl = "0.14"
gl-headless = "0.2"
```

```rust
use gl_headless::gl_headless;

#[gl_headless]
unsafe fn main() {
    let (mut major, mut minor) = (0, 0);
    gl::GetIntegerv(gl::MAJOR_VERSION, &mut major);
    gl::GetIntegerv(gl::MINOR_VERSION, &mut minor);
    println!("OpenGL {major}.{minor}");
}
```

## Specify OpenGL Version

By default <code>#[[gl_headless]]</code> attempts to create an OpenGL 4.6 context. To use a specific version add, e.g. `version = "3.3"`:

```rust
use gl_headless::gl_headless;

#[gl_headless(version = "3.3")]
unsafe fn main() {
    let (mut major, mut minor) = (0, 0);
    gl::GetIntegerv(gl::MAJOR_VERSION, &mut major);
    gl::GetIntegerv(gl::MINOR_VERSION, &mut minor);
    println!("OpenGL {major}.{minor}");
}
```

## Parameters & Return Type

Specify function parameters and return type as you otherwise would:

```rust
use gl_headless::gl_headless;

fn main() {
    let version = get_version("OpenGL");
    println!("{version}");
}

#[gl_headless]
fn get_version(prefix: &str) -> String {
    let (mut major, mut minor) = (0, 0);
    unsafe {
        gl::GetIntegerv(gl::MAJOR_VERSION, &mut major);
        gl::GetIntegerv(gl::MINOR_VERSION, &mut minor);
    }
    format!("{prefix} {major}.{minor}")
}
```

## Multiple Functions

Multiple functions can use <code>#[[gl_headless]]</code>:

```rust
use gl_headless::gl_headless;

fn main() {
    unsafe {
        example1();
        example2();
    }
}

#[gl_headless(version = "3.3")]
unsafe fn example1() {
    let (mut major, mut minor) = (0, 0);
    gl::GetIntegerv(gl::MAJOR_VERSION, &mut major);
    gl::GetIntegerv(gl::MINOR_VERSION, &mut minor);
    println!("OpenGL {major}.{minor}");
}

#[gl_headless]
unsafe fn example2() {
    let mut handle = 0;
    gl::CreateBuffers(1, &mut handle);

    let data: [f32; 5] = [1.0, 2.0, 3.0, 4.0, 5.0];
    gl::NamedBufferData(
        handle,
        std::mem::size_of_val(&data) as _,
        data.as_ptr() as *const _,
        gl::STATIC_DRAW,
    );

    let mut byte_size = 0;
    gl::GetNamedBufferParameteriv(handle, gl::BUFFER_SIZE, &mut byte_size);
    let float_count = (byte_size as usize) / std::mem::size_of::<f32>() as usize;

    let mut floats = vec![0.0_f32; float_count];
    gl::GetNamedBufferSubData(
        handle,
        0,
        std::mem::size_of_val(floats.as_slice()) as _,
        floats.as_mut_ptr() as *mut _,
    );

    println!("Write: {:?}", data);
    println!("Read:  {:?}", floats);

    assert_eq!(data, floats.as_slice());
}
```

[gl_headless]: https://docs.rs/gl-headless/*/gl_headless/attr.gl_headless.html
