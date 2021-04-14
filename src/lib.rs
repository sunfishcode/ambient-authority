//! Ambient Authority
//!
//! Capability-oriented library APIs should re-export the [`ambient_authority`]
//! function and [`AmbientAuthority`] type in their own API, and include this
//! type as an argument to any function which utilizes ambient authority.
//!
//! For example:
//!
//! ```rust
//! // Re-export the `AmbientAuthority` type.
//! pub use ambient_authority::{ambient_authority, AmbientAuthority};
//!
//! // Declare functions that use ambient authority with an `AmbientAuthority`
//! // argument.
//! pub fn do_a_thing(_: AmbientAuthority) {
//!     println!("hello world");
//! }
//! ```
//!
//! To use an API that uses [`AmbientAuthority`], call [`ambient_authority`]
//! and pass the result:
//!
//! ```rust
//! do_a_thing(ambient_authority());
//! ```

#![deny(missing_docs)]
#![forbid(unsafe_code)]
#![no_std]

/// Instances of this `AmbientAuthority` type serve to indicate that the
/// [`ambient_authority`] function has been called, indicating that the user
/// has explicitly opted into using ambient authority.
#[derive(Copy, Clone)]
pub struct AmbientAuthority(());

/// Return an `AmbientAuthority` value, which allows use of functions that
/// include an `AmbientAuthority` argument.
#[inline]
#[must_use]
#[allow(clippy::missing_const_for_fn)]
pub fn ambient_authority() -> AmbientAuthority {
    AmbientAuthority(())
}
