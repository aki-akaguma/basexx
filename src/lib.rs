/*!
**`basexx`** is a Rust library that provides multiple base encoding and decoding functionalities.

### Core Features:

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

### Modules:

*   **`ags`:** This module provides the `AsciiGraphicSet` struct, which is used to manage the character sets for the various base encodings. It includes functions for converting between binary and ASCII representations, with SIMD-accelerated versions for improved performance.
*   **`base32`:** This module implements the Base32 encoding and decoding functionality.
*   **`base32i`:** This module provides an integer-based implementation of Base32, utilizing the `num-bigint` crate.
*   **`base56`:** This module implements the Base56 encoding and decoding functionality.
*   **`base58`:** This module implements the Base58 encoding and decoding functionality.
*   **`base58b`:** This module provides a Bitcoin-style implementation of Base58.
*   **`base58r`:** This module offers a `rug`-based implementation of Base58 for arbitrary-precision arithmetic.
*   **`base64`:** This module implements the Base64 encoding and decoding functionality.
*   **`base64g`:** This module provides a Base64 implementation with padding.

### Usage:

Each base encoding is exposed as a struct (e.g., `Base32`, `Base64`) that can be instantiated with a default or custom character map. The `encode` and `decode` methods can then be used to perform the respective operations.

**Example:**

```rust
use basexx::Base64;

let data = b"Hello, world!";
let base64 = Base64::new();
let encoded = base64.encode(data).unwrap();
println!("Encoded: {}", encoded);
let decoded = base64.decode(&encoded).unwrap();
assert_eq!(data, &decoded[..]);
```
*/

//mod aligned_data;
//#[allow(unused_imports)]
//use aligned_data::*;

mod ags;
use ags::*;

mod base32;
mod base32i;
mod base64;
mod base64g;

mod base56;
mod base58;
mod base58b;

pub use base32::*;
pub use base32i::*;
pub use base64::*;
pub use base64g::*;

pub use base56::*;
pub use base58::*;
pub use base58b::*;

#[cfg(feature = "rug")]
mod base58r;

#[cfg(feature = "rug")]
pub use base58r::*;

#[derive(Debug, PartialEq)]
pub enum EncodeError {
    InvalidIndex(u8),
}

#[derive(Debug, PartialEq)]
pub enum DecodeError {
    InvalidByte(u8),
    InvalidLength(usize),
    OutputNumberTooBig(u32, String),
}

#[cfg(test)]
mod test_utils;
