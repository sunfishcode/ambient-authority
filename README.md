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
type [`AmbientAuthority`], which is an empty type used in APIs to declare that
they use ambient authority.

The convention for a crate to declare that its API is capability-oriented is:
 - Re-export the [`ambient_authority`] function and [`AmbientAuthority`] type
   from this crate.
 - Add an `AmbientAuthority` argument to any function in its API which uses
   ambient authority.

The instructions for a user wishing to only use capability-oriented crates are:
 - Ensure that all immediate dependencies have a re-export of
   [`ambient_authority`] and [`AmbientAuthority`] from this crate, which
   indicates their intent to provide a capability-oriented API.
 - Copy the clippy/clippy.toml file into their top level source directory, add
   `#![deny(clippy::disallowed_method)]` to their main.rs or lib.rs, and run
   `cargo +nightly clippy` or equivalent.

[`AmbientAuthority`]: https://docs.rs/ambient-authority/latest/ambient_authority/struct.AmbientAuthority.html
[`ambient_authority`]: https://docs.rs/ambient-authority/latest/ambient_authority/func.ambient_authority.html
