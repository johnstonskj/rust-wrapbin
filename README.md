# Package `wrapbin`

Simple binary _newtype_ as wrapped `Cow` (copy-on-write) `u8` (byte) array.

[![Apache-2.0 License](https://img.shields.io/badge/License-Apache_2.0-blue.svg)](https://opensource.org/licenses/Apache-2.0)
[![MIT License](https://img.shields.io/badge/license-mit-118811.svg)](https://opensource.org/license/mit)
[![Rust Workflow](https://github.com/johnstonskj/rust-wrapbin/actions/workflows/rust.yml/badge.svg)](<https://github.com/johnstonskj/rust-wrapbin/actions/workflows/rust.yml>)
[![Security Audit Workflow](https://github.com/johnstonskj/rust-wrapbin/actions/workflows/security-audit.yml/badge.svg)](<https://github.com/johnstonskj/rust-wrapbin/actions/workflows/security-audit.yml>)
[![Coverage Status](https://codecov.io/github/johnstonskj/rust-wrapbin/graph/badge.svg?token=TcyByrr7By)](<https://codecov.io/github/johnstonskj/rust-wrapbin>)
[![crates.io](https://img.shields.io/crates/v/wrapbin.svg)](https://crates.io/crates/wrapbin)
[![docs.rs](https://docs.rs/xml_dom/badge.svg)](https://docs.rs/wrapbin)
[![GitHub stars](https://img.shields.io/github/stars/johnstonskj/rust-wrapbin.svg)](<https://github.com/johnstonskj/rust-wrapbin/stargazers>)

This simple wrapper allows for conversion from different types that provide
`as_bytes`, `to_bytes`, `into_bytes` and other methods into a common `Binary`
type. By choosing to use `Cow<[u8]>` as the implementation, we can also respect
the manner in which we received the bytes from the providing type/value. So,
where a `&str` value has `as_bytes()` returning `&[u8]` we store the borrowed
value; however, where a `String`'s `into_bytes()` method returns `Vec<u8>` we
store the owned value.

For information on contributing to this project, see the following.

1. Project [Code of Conduct](https://github.com/johnstonskj/rust-wrapbin/blob/main/CODE_OF_CONDUCT.md).
1. Project [Contribution Guidelines](https://github.com/johnstonskj/rust-wrapbin/blob/main/CONTRIBUTING.md).
1. Project [TODO Items](<https://github.com/johnstonskj/rust-wrapbin/issues>) in Issues.
1. Repository [Change Log](https://github.com/johnstonskj/rust-wrapbin/blob/main/CHANGELOG.md).

# Features

- **alloc**; Requires the Rust `alloc` crate when built as `no_std`. **Default**.
- **std**; Build with the standard library.
- **fmt**; Adds support for the format specifiers in the `std::fmt` module:
  `Binary`, `LowerHex`, `Octal`, and `UpperHex`. This **requires** the
  _repr-array_ feature. **Default**.
- Representation formats:
  - **repr-array**; Array representation; e.g. `0x[01, 0e, b2, 8c]`. **Default**.
  - **repr-base64**; Base64 representation.
  - **repr-dump**; Dump representation.
  - **repr-string**; String representation; e.g. `0x"01_0e_b2_8c"`.
  - **repr-color**; Adds color to the representations above.

# Examples

## Construction

Store bytes from three string types.

```rust
use wrapbin::Binary;

let bin_1 = Binary::from("Hello World!".to_string());
let bin_2 = Binary::from("Hello World!");
let bin_3 = Binary::from(b"Hello World!");
assert_eq!(bin_1, bin_2);
assert_eq!(bin_2, bin_3);
```

While this is relatively obvious for ASCII characters such as those above the
byte mapping isn't quite so straightforward for more complex scripts with
multi-byte characters _and_ multi-glyph, stacking, characters (one single
character below is actually four).

```rust
use wrapbin::Binary;

let binary = Binary::from("༄༏ༀ་མ་ཎིཔ་དྨེ་ཧྤུྂ།།");
assert_eq!(binary.len(), 60);
```

## Feature `fmt`

When the feature `fmt` is enabled the `Binary` type also supports display
formatting using standard Rust numeric format specifiers 'b', 'o', 'x' and
'X'.

```rust
use wrapbin::Binary;

let binary = Binary::from("Hello World!");

assert_eq!(
    format!("{binary:b}"),
    "0b[01001000, 01100101, 01101100, 01101100, 01101111, 00100000, 01010111, 01101111, 01110010, 01101100, 01100100, 00100001]"
);
assert_eq!(
    format!("{binary:o}"),
    "0o[110, 145, 154, 154, 157, 040, 127, 157, 162, 154, 144, 041]"
);
assert_eq!(
    format!("{binary}"),
    "0d[072, 101, 108, 108, 111, 032, 087, 111, 114, 108, 100, 033]"
);
assert_eq!(
    format!("{binary:x}"),
    "0x[48, 65, 6c, 6c, 6f, 20, 57, 6f, 72, 6c, 64, 21]"
);
assert_eq!(
    format!("{binary:X}"),
    "0X[48, 65, 6C, 6C, 6F, 20, 57, 6F, 72, 6C, 64, 21]"
);
```

The '#' flag for alternate representation enables _compact_ mode which
removes spaces and padding from the generated output. The formatted output
uses the Array representation enabled by the `repr-array` feature.

```rust
use wrapbin::Binary;

let binary = Binary::from("Hello World!");

assert_eq!(
    format!("{binary:#b}"),
    "0b[1001000,1100101,1101100,1101100,1101111,100000,1010111,1101111,1110010,1101100,1100100,100001]"
);
assert_eq!(
    format!("{binary:#o}"),
    "0o[110,145,154,154,157,40,127,157,162,154,144,41]"
);
assert_eq!(
    format!("{binary:#}"),
    "0d[72,101,108,108,111,32,87,111,114,108,100,33]"
);
assert_eq!(
    format!("{binary:#x}"),
    "0x[48,65,6c,6c,6f,20,57,6f,72,6c,64,21]"
);
assert_eq!(
    format!("{binary:#X}"),
    "0X[48,65,6C,6C,6F,20,57,6F,72,6C,64,21]"
);
```

## Testing and Coverage

Current Coverage map:

<https://codecov.io/github/johnstonskj/rust-wrapbin/graphs/icicle.svg?token=TcyByrr7By>

## License(s)

The contents of this repository are made available under the following
licenses:

### Apache-2.0

> ```text
> Copyright 2025 Simon Johnston <johnstonskj@gmail.com>
> 
> Licensed under the Apache License, Version 2.0 (the "License");
> you may not use this file except in compliance with the License.
> You may obtain a copy of the License at
> 
>     http://www.apache.org/licenses/LICENSE-2.0
> 
> Unless required by applicable law or agreed to in writing, software
> distributed under the License is distributed on an "AS IS" BASIS,
> WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
> See the License for the specific language governing permissions and
> limitations under the License.
> ```

See the enclosed file [LICENSE-Apache](https://github.com/johnstonskj/rust-wrapbin/blob/main/LICENSE-Apache).

### MIT

> ```text
> Copyright 2025 Simon Johnston <johnstonskj@gmail.com>
> 
> Permission is hereby granted, free of charge, to any person obtaining a copy
> of this software and associated documentation files (the “Software”), to deal
> in the Software without restriction, including without limitation the rights to
> use, copy, modify, merge, publish, distribute, sublicense, and/or sell copies of
> the Software, and to permit persons to whom the Software is furnished to do so,
> subject to the following conditions:
> 
> The above copyright notice and this permission notice shall be included in all
> copies or substantial portions of the Software.
> 
> THE SOFTWARE IS PROVIDED “AS IS”, WITHOUT WARRANTY OF ANY KIND, EXPRESS OR IMPLIED,
> INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY, FITNESS FOR A
> PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR COPYRIGHT
> HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION
> OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN CONNECTION WITH THE
> SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE.
> ```

See the enclosed file [LICENSE-MIT](https://github.com/johnstonskj/rust-wrapbin/blob/main/LICENSE-MIT).

### Contributions

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall
be dual licensed as above, without any additional terms or conditions.

See [CONTRIBUTING](https://github.com/johnstonskj/rust-wrapbin/blob/main/CONTRIBUTING.md) and the
project's [CODE_OF_CONDUCT](https://github.com/johnstonskj/rust-wrapbin/blob/main/CODE_OF_CONDUCT.md).
