<div align="center">
  <h1><code>ambient-authority</code></h1>

  <p>
    <strong>Ambient Authority</strong>
  </p>

  <p>
    <a href="https://github.com/sunfishcode/ambient-authority/actions?query=workflow%3ACI"><img src="https://github.com/sunfishcode/ambient-authority/workflows/CI/badge.svg" alt="Github Actions CI Status" /></a>
    <a href="https://crates.io/crates/ambient-authority"><img src="https://img.shields.io/crates/v/ambient-authority.svg" alt="crates.io page" /></a>
    <a href="https://docs.rs/ambient-authority"><img src="https://docs.rs/ambient-authority/badge.svg" alt="docs.rs docs" /></a>
  </p>
</div>

In capability-based security context, *ambient authority* means anything a
program can do that interacts with the outside world that isn't represented by
a handle.

This crate defines a function [`ambient_authority`] which returns a value of
type [`AmbientAuthority`], which is an empty type used in function signatures
to declare that they use ambient authority.

The convention for a crate to declare that its API avoids ambient authority is:
 - If the crate wishes to have functions which use ambient authority, adding an
   [`AmbientAuthority`] argument to them, along with a `# Ambient Authority`
   section in their documentation comments explaining their use of ambient
   authority, and re-exporting the [`ambient_authority`] function and
   `AmbientAuthority` type from this crate.

 - Ensure that all other `pub` functions avoid using ambient authority,
   including mutable static state such as static `Atomic`, `Cell`, `RefCell`,
   `Mutex`, `RwLock`, or similar state, including `once_cell` or `lazy_static`
   state with initialization that uses ambient authority.

One of the cool things about capability-oriented APIs is that programs don't
need to be pure to take advantage of them. That said, for programs which do which
to aim for purity, this repository has a clippy configuration which can help:
 - Manually ensure that all immediate dependencies follow the above convention.
 - Copy the [clippy.toml] file into the top level source directory, add
   `#![deny(clippy::disallowed_method)]` to the root module (main.rs or lib.rs),
   and run `cargo +nightly clippy` or equivalent.

[clippy.toml]: https://github.com/sunfishcode/ambient-authority/blob/main/clippy.toml
[`AmbientAuthority`]: https://docs.rs/ambient-authority/latest/ambient_authority/struct.AmbientAuthority.html
[`ambient_authority`]: https://docs.rs/ambient-authority/latest/ambient_authority/func.ambient_authority.html
