# basexx

[![crate][crate-image]][crate-link]
[![Docs][docs-image]][docs-link]
![Rust Version][rustc-image]
![Apache2/MIT licensed][license-image]
[![Test ubu][test-ubuntu-image]][test-ubuntu-link]
[![Test mac][test-windows-image]][test-windows-link]
[![Test win][test-macos-image]][test-macos-link]

**`basexx`** is a Rust library that provides multiple base encoding and decoding functionalities.

#### Core Features:

*   **Base Encodings:** The library supports the following base encodings:
    *   Base32
    *   Base32I (integer-based)
    *   Base56
    *   Base58
    *   Base58B (Bitcoin-style)
    *   Base58R (Rug-based)
    *   Base64
    *   Base64G (with padding)
*   **SIMD Acceleration:** The library leverages SIMD instructions (SSSE3 and AVX2) for improved performance on compatible x86/x86_64 architectures.
*   **Custom Character Maps:** Users can provide their own character maps for each encoding, allowing for customized implementations.
*   **Error Handling:** The library defines `EncodeError` and `DecodeError` enums to handle potential errors during the encoding and decoding processes.

#### Modules:

*   **`ags`:** This module provides the `AsciiGraphicSet` struct, which is used to manage the character sets for the various base encodings. It includes functions for converting between binary and ASCII representations, with SIMD-accelerated versions for improved performance.
*   **`base32`:** This module implements the Base32 encoding and decoding functionality.
*   **`base32i`:** This module provides an integer-based implementation of Base32, utilizing the `num-bigint` crate.
*   **`base56`:** This module implements the Base56 encoding and decoding functionality.
*   **`base58`:** This module implements the Base58 encoding and decoding functionality.
*   **`base58b`:** This module provides a Bitcoin-style implementation of Base58.
*   **`base58r`:** This module offers a `rug`-based implementation of Base58 for arbitrary-precision arithmetic.
*   **`base64`:** This module implements the Base64 encoding and decoding functionality.
*   **`base64g`:** This module provides a Base64 implementation with padding.
*   **`aligned_data`:** This module provides helper functions for creating aligned data structures, which can improve performance in certain SIMD operations.

#### Usage:

Each base encoding is exposed as a struct (e.g., `Base32`, `Base64`) that can be instantiated with a default or custom character map. The `encode` and `decode` methods can then be used to perform the respective operations.

**Example:**

```rust
use basexx::Base64;

fn main() {
    let data = b"Hello, world!";
    let base64 = Base64::new();
    let encoded = base64.encode(data).unwrap();
    println!("Encoded: {}", encoded);
    let decoded = base64.decode(&encoded).unwrap();
    assert_eq!(data, &decoded[..]);
}
```

# Changelogs

[This crate's changelog here.](https://github.com/aki-akaguma/basexx/blob/main/CHANGELOG.md)

# License

This project is licensed under either of

 * Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or
   https://www.apache.org/licenses/LICENSE-2.0)
 * MIT license ([LICENSE-MIT](LICENSE-MIT) or
   https://opensource.org/licenses/MIT)

at your option.

[//]: # (badges)

[crate-image]: https://img.shields.io/crates/v/basexx.svg
[crate-link]: https://crates.io/crates/basexx
[docs-image]: https://docs.rs/basexx/badge.svg
[docs-link]: https://docs.rs/basexx/
[rustc-image]: https://img.shields.io/badge/rustc-1.74+-blue.svg
[license-image]: https://img.shields.io/badge/license-Apache2.0/MIT-blue.svg
[test-ubuntu-image]: https://github.com/aki-akaguma/basexx/actions/workflows/test-ubuntu.yml/badge.svg
[test-ubuntu-link]: https://github.com/aki-akaguma/basexx/actions/workflows/test-ubuntu.yml
[test-macos-image]: https://github.com/aki-akaguma/basexx/actions/workflows/test-macos.yml/badge.svg
[test-macos-link]: https://github.com/aki-akaguma/basexx/actions/workflows/test-macos.yml
[test-windows-image]: https://github.com/aki-akaguma/basexx/actions/workflows/test-windows.yml/badge.svg
[test-windows-link]: https://github.com/aki-akaguma/basexx/actions/workflows/test-windows.yml
