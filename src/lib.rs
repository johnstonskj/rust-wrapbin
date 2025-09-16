/*!
Simple binary *newtype* as wrapped `Cow` (copy-on-write) `u8` (byte) array.

This is intended as a useful abstraction for a binary string, or an array of
bytes. As such it can be initialized from a `Vec<u8>`, `&[u8]`, or `b""`
literal or any type implementing Into<


# Example

```rust
```

# Features

- **alloc**; Requires the Rust `alloc` crate when built as `no_std`. **Default**.
- **std**; Build with the standard library.
- **fmt**; Adds support for the format specifiers in the `std::fmt` module:
  `Binary`, `LowerHex`, `Octal`, and `UpperHex`. This requires the
  *repr-array* feature. **Default**.
- Representation formats:
  - **repr-array**; Array representation; e.g. `0x[01, 0e, b2, 8c]`. **Default**.
  - **repr-base64**; Base64 representation.
  - **repr-dump**; Dump representation.
  - **repr-string**; String representation; e.g. `0x"01_0e_b2_8c"`.
  - **repr-color**; Adds color to the representations above.
*/

#![warn(
    unknown_lints,
    // ---------- Stylistic
    absolute_paths_not_starting_with_crate,
    elided_lifetimes_in_paths,
    explicit_outlives_requirements,
    macro_use_extern_crate,
    nonstandard_style, /* group */
    noop_method_call,
    rust_2018_idioms,
    single_use_lifetimes,
    trivial_casts,
    trivial_numeric_casts,
    // ---------- Future
    future_incompatible, /* group */
    rust_2021_compatibility, /* group */
    // ---------- Public
    missing_debug_implementations,
    // missing_docs,
    unreachable_pub,
    // ---------- Unsafe
    unsafe_code,
    unsafe_op_in_unsafe_fn,
    // ---------- Unused
    unused, /* group */
    // ---------- Clippy - no_std
    //clippy::alloc_instead_of_core,
    clippy::std_instead_of_alloc,
    clippy::std_instead_of_core,
)]
#![deny(
    // ---------- Public
    exported_private_dependencies,
    // ---------- Deprecated
    anonymous_parameters,
    bare_trait_objects,
    ellipsis_inclusive_range_patterns,
    // ---------- Unsafe
    deref_nullptr,
    drop_bounds,
    dyn_drop,
)]
#![no_std]

#[cfg(any(test, feature = "alloc", not(feature = "std")))]
extern crate alloc;
/*
 * The following is no longer supported as there is no known external crate named
 * `std` after approximately edition 2018. However, it does make a nice logical
 * follow-on from the line above for the `alloc` crate.
 *
 *  ```
 * #[cfg(any(test, feature = "std"))]
 * extern crate std;
 * ```
 */

use alloc::{
    borrow::{Borrow, Cow},
    string::String,
    vec::Vec,
};
use core::{
    convert::{AsRef, From},
    default::Default,
    iter::{FromIterator, IntoIterator, Iterator},
    ops::Deref,
    option::Option,
};
#[cfg(feature = "fmt")]
use core::{
    fmt::{Formatter, Result as FmtResult},
    write,
};

// ------------------------------------------------------------------------------------------------
// Public Type ❱ Binary
// ------------------------------------------------------------------------------------------------

#[derive(Clone, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Binary<'a>(Cow<'a, [u8]>);

// ------------------------------------------------------------------------------------------------
// Implementations ❱ From/Into Binary inner representations
// ------------------------------------------------------------------------------------------------

impl<'a> From<Cow<'a, [u8]>> for Binary<'a> {
    fn from(value: Cow<'a, [u8]>) -> Self {
        Self(value)
    }
}

impl From<Vec<u8>> for Binary<'_> {
    fn from(value: Vec<u8>) -> Self {
        Self(Cow::Owned(value))
    }
}

impl<'a> From<&'a [u8]> for Binary<'a> {
    fn from(value: &'a [u8]) -> Self {
        Self(Cow::Borrowed(value))
    }
}

impl<'a> From<Binary<'a>> for Cow<'a, [u8]> {
    fn from(value: Binary<'a>) -> Self {
        value.0
    }
}

impl<'a> From<Binary<'a>> for Vec<u8> {
    fn from(value: Binary<'a>) -> Self {
        value.0.into_owned()
    }
}

// ------------------------------------------------------------------------------------------------
// Implementations ❱ From/Into Byte Iterators
// ------------------------------------------------------------------------------------------------

impl FromIterator<u8> for Binary<'_> {
    fn from_iter<T: IntoIterator<Item = u8>>(iter: T) -> Self {
        Self(Cow::Owned(iter.into_iter().collect()))
    }
}

impl<'a> FromIterator<&'a u8> for Binary<'a> {
    fn from_iter<T: IntoIterator<Item = &'a u8>>(iter: T) -> Self {
        Self(Cow::Owned(iter.into_iter().copied().collect()))
    }
}

impl IntoIterator for Binary<'_> {
    type Item = u8;
    type IntoIter = alloc::vec::IntoIter<u8>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_owned().into_iter()
    }
}

impl<'a> IntoIterator for &'a Binary<'a> {
    type Item = &'a u8;
    type IntoIter = alloc::slice::Iter<'a, u8>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.as_ref().iter()
    }
}

// ------------------------------------------------------------------------------------------------
// Implementations ❱ From Binary adjacent types
// ------------------------------------------------------------------------------------------------

impl From<String> for Binary<'_> {
    fn from(value: String) -> Self {
        Binary::from(value.into_bytes())
    }
}

impl<'a, 'b: 'a> From<&'b str> for Binary<'a> {
    fn from(value: &'b str) -> Self {
        Binary::from(value.as_bytes())
    }
}

// ------------------------------------------------------------------------------------------------
// Implementations ❱ From Primative types
// ------------------------------------------------------------------------------------------------

impl From<u8> for Binary<'_> {
    fn from(value: u8) -> Self {
        Binary::from(value.to_ne_bytes().to_vec())
    }
}

impl From<u16> for Binary<'_> {
    fn from(value: u16) -> Self {
        Binary::from(value.to_ne_bytes().to_vec())
    }
}

impl From<u32> for Binary<'_> {
    fn from(value: u32) -> Self {
        Binary::from(value.to_ne_bytes().to_vec())
    }
}

impl From<u64> for Binary<'_> {
    fn from(value: u64) -> Self {
        Binary::from(value.to_ne_bytes().to_vec())
    }
}

impl From<u128> for Binary<'_> {
    fn from(value: u128) -> Self {
        Binary::from(value.to_ne_bytes().to_vec())
    }
}

impl From<usize> for Binary<'_> {
    fn from(value: usize) -> Self {
        Binary::from(value.to_ne_bytes().to_vec())
    }
}

impl From<i8> for Binary<'_> {
    fn from(value: i8) -> Self {
        Binary::from(value.to_ne_bytes().to_vec())
    }
}

impl From<i16> for Binary<'_> {
    fn from(value: i16) -> Self {
        Binary::from(value.to_ne_bytes().to_vec())
    }
}

impl From<i32> for Binary<'_> {
    fn from(value: i32) -> Self {
        Binary::from(value.to_ne_bytes().to_vec())
    }
}

impl From<i64> for Binary<'_> {
    fn from(value: i64) -> Self {
        Binary::from(value.to_ne_bytes().to_vec())
    }
}

impl From<i128> for Binary<'_> {
    fn from(value: i128) -> Self {
        Binary::from(value.to_ne_bytes().to_vec())
    }
}

impl From<isize> for Binary<'_> {
    fn from(value: isize) -> Self {
        Binary::from(value.to_ne_bytes().to_vec())
    }
}

impl From<f32> for Binary<'_> {
    fn from(value: f32) -> Self {
        Binary::from(value.to_ne_bytes().to_vec())
    }
}

impl From<f64> for Binary<'_> {
    fn from(value: f64) -> Self {
        Binary::from(value.to_ne_bytes().to_vec())
    }
}

impl From<char> for Binary<'_> {
    fn from(value: char) -> Self {
        let mut buffer = [0_u8; 4];
        value.encode_utf8(&mut buffer);
        Binary::from(buffer.to_vec())
    }
}

impl From<bool> for Binary<'_> {
    fn from(value: bool) -> Self {
        Binary::from(u8::from(value))
    }
}

// ------------------------------------------------------------------------------------------------
// Implementations ❱ From useful library types
// ------------------------------------------------------------------------------------------------

impl From<core::net::Ipv4Addr> for Binary<'_> {
    fn from(value: core::net::Ipv4Addr) -> Self {
        Binary::from(value.octets().to_vec())
    }
}

impl From<core::net::Ipv6Addr> for Binary<'_> {
    fn from(value: core::net::Ipv6Addr) -> Self {
        Binary::from(value.octets().to_vec())
    }
}

impl<'a, 'b: 'a> From<&'b core::ffi::CStr> for Binary<'a> {
    fn from(value: &'b core::ffi::CStr) -> Self {
        Binary::from(value.to_bytes_with_nul())
    }
}

// ------------------------------------------------------------------------------------------------
// Implementations ❱ Reference
// ------------------------------------------------------------------------------------------------

impl AsRef<[u8]> for Binary<'_> {
    fn as_ref(&self) -> &[u8] {
        match &self.0 {
            Cow::Borrowed(v) => v,
            Cow::Owned(v) => v.as_ref(),
        }
    }
}

impl Borrow<[u8]> for Binary<'_> {
    fn borrow(&self) -> &[u8] {
        match &self.0 {
            Cow::Borrowed(v) => v,
            Cow::Owned(v) => v.as_ref(),
        }
    }
}

impl Deref for Binary<'_> {
    type Target = [u8];

    fn deref(&self) -> &Self::Target {
        match &self.0 {
            Cow::Borrowed(v) => v,
            Cow::Owned(v) => v.as_ref(),
        }
    }
}

// ------------------------------------------------------------------------------------------------
// Implementations ❱ Format
// ------------------------------------------------------------------------------------------------

#[cfg(feature = "fmt")]
use crate::repr::array::{ArrayFormatOptions, array_representation};

#[cfg(feature = "fmt")]
impl core::fmt::Display for Binary<'_> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(
            f,
            "{}",
            array_representation(
                self,
                &ArrayFormatOptions::default()
                    .with_decimal_bytes()
                    .compact(f.alternate())
            )
        )
    }
}

#[cfg(feature = "fmt")]
impl core::fmt::Binary for Binary<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(
            f,
            "{}",
            array_representation(
                self,
                &ArrayFormatOptions::default()
                    .with_binary_bytes()
                    .compact(f.alternate())
            )
        )
    }
}

#[cfg(feature = "fmt")]
impl core::fmt::Octal for Binary<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(
            f,
            "{}",
            array_representation(
                self,
                &ArrayFormatOptions::default()
                    .with_octal_bytes()
                    .compact(f.alternate())
            )
        )
    }
}

#[cfg(feature = "fmt")]
impl core::fmt::LowerHex for Binary<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(
            f,
            "{}",
            array_representation(
                self,
                &ArrayFormatOptions::default()
                    .with_lower_hex_bytes()
                    .compact(f.alternate())
            )
        )
    }
}

#[cfg(feature = "fmt")]
impl core::fmt::UpperHex for Binary<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(
            f,
            "{}",
            array_representation(
                self,
                &ArrayFormatOptions::default()
                    .with_upper_hex_bytes()
                    .compact(f.alternate())
            ),
        )
    }
}

// ------------------------------------------------------------------------------------------------
// Implementation ❱ Binary
// ------------------------------------------------------------------------------------------------

impl Binary<'_> {
    // --------------------------------------------------------------------------------------------
    // Cow Access
    // --------------------------------------------------------------------------------------------

    pub fn into_owned(self) -> Vec<u8> {
        self.0.into_owned()
    }

    pub fn is_borrowed(&self) -> bool {
        matches!(self.0, Cow::Borrowed(_))
    }

    pub fn is_owned(&self) -> bool {
        matches!(self.0, Cow::Owned(_))
    }

    pub fn to_mut(&mut self) -> &mut Vec<u8> {
        self.0.to_mut()
    }

    // --------------------------------------------------------------------------------------------
    // Vector Access
    // --------------------------------------------------------------------------------------------

    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn iter(&self) -> impl Iterator<Item = &u8> {
        self.0.iter()
    }
    pub fn as_slice(&self) -> &[u8] {
        match &self.0 {
            Cow::Borrowed(v) => v,
            Cow::Owned(v) => v.as_slice(),
        }
    }

    pub fn push(&mut self, byte: u8) {
        self.0.to_mut().push(byte)
    }

    pub fn pop(&mut self) -> Option<u8> {
        self.0.to_mut().pop()
    }

    pub fn insert(&mut self, index: usize, byte: u8) {
        self.0.to_mut().insert(index, byte)
    }

    pub fn remove(&mut self, index: usize) -> u8 {
        self.0.to_mut().remove(index)
    }

    pub fn clear(&mut self) {
        self.0.to_mut().clear()
    }
}

// ------------------------------------------------------------------------------------------------
// Modules
// ------------------------------------------------------------------------------------------------

pub mod error;

pub mod repr;
