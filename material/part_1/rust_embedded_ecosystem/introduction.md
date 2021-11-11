# Exploring the ecosystem

Ever since the Rust project decided it would be a low-level language that contents with C and C++ ([this hasn't always been the case](https://en.wikipedia.org/wiki/Rust_(programming_language)#History)), it fully committed to it.
The result of this is that support for bare metal project is almost as good as for normal projects.

Pretty much all of the normal tools are available for us and even the standard library is split up to help us.

## Std, Core and Alloc

Rust has a (purposefully) small standard library called `std`. It contains many of the things you'd expect like:
- Generic collections
- Sockets
- Strings and formatting
- Error handling
- Threading
- And more, feel free to browse the docs: [https://doc.rust-lang.org/std/index.html](https://doc.rust-lang.org/std/index.html)

Many of these are not suitable for use in embedded systems.
We're likely to not even have an allocator!

To deal with this gracefully, the standard library has been split up into three parts.
- `core`: Contains all primitives and everything that that does not require an OS or an allocator
- `alloc`: Contains most dynamic collection types, String, Box (unique_ptr), RC (shared_ptr) and more
- `std`: Reexports core and alloc and adds the normal on OS depending API's.

A crate (rust library) can choose to not require std by having the `#![no_std]` attribute in the source code.
From then on, only core and possibly alloc can be used.
Many relevant crates have a feature flag for std. If that's off, then the core functionality can be used by embedded applications.

## Packages and cargo

Rust has a package manager called Cargo which can also be used for embedded projects.
You can set up the build target so it will build for the used microcontroller architecture.

This all uses the standard Rust workflow, so to add a new crate, for example one that has a stack-allocated vector type,
you can just add one line to your `Cargo.toml` file:

```toml
[dependencies]
# ... list of other dependencies
arrayvec = { version = "0.7.2", default-features = false }
```
*In this case we need to turn off the default features, because by default the std feature is on.*

## Probe-rs

Another part of especially the embedded ecosystem to highlight is probe-rs.
This is a relatively new library that contents with jlink and openocd.
Many tools are built around it like [`cargo flash`](https://crates.io/crates/cargo-flash) and [`probe-run`](https://crates.io/crates/probe-run).
