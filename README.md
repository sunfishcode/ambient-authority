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
   [`AmbientAuthority`] argument to them, and re-exporting the
   [`ambient_authority`] function and `AmbientAuthority` type from this crate.

 - Ensure that all other `pub` functions avoid using ambient authority,
   including mutable static state such as static `Atomic` state, static
   `Cell`s or `RefCell`s, or `once_cell` or `lazy_static` state with
   initialization that uses ambient authority.

The instructions for a user wishing to only use capability-oriented crates are:
 - Manually ensure that all immediate dependencies follow the above convention.
 - Copy the clippy/clippy.toml file into their top level source directory, add
   `#![deny(clippy::disallowed_method)]` to their main.rs or lib.rs, and run
   `cargo +nightly clippy` or equivalent.

[`AmbientAuthority`]: https://docs.rs/ambient-authority/latest/ambient_authority/struct.AmbientAuthority.html
[`ambient_authority`]: https://docs.rs/ambient-authority/latest/ambient_authority/func.ambient_authority.html
