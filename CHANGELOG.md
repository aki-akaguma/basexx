# Changelog: basexx

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]


## [0.2.0] (2026-05-26)
### Added
- Introduce `bytemuck` crate for safer memory casting and alignment handling.

### Fixed
- Resolve potential Undefined Behavior (UB) in SIMD implementations by replacing unaligned pointer dereferences with safe `bytemuck` primitives and `std::ptr::copy_nonoverlapping`.
- Fix broken benchmark build caused by missing legacy `aligned_data` module and missing macro definitions.
- Improve safety of `from_utf8_unchecked` usage by adding `debug_assert!` validation and safety documentation across all encoding modules.
- Eliminate numerous deprecation warnings in benchmarks by migrating from `criterion::black_box` to `std::hint::black_box`.

### Changed
- Unify runtime CPU feature detection logic using a new `x86_dispatch!` macro, reducing code duplication in `ags`, `base32`, and `base64` modules.
- Rename `aligned_data` feature to `aligned-vec` to match the actual dependency and clarify its purpose.
- Refactor SIMD chunk processing logic to use safe slice iterators (`chunks_exact`) for better readability and robustness.
- update crate: criterion(0.8)
- minimum test version 1.86.0 on `.github/workflows`

## [0.1.1] (2025-09-24)
### Added
* `specs`
* more tests

### Fixed
* `unused_imports`

## [0.1.0] (2025-08-xx)
* first commit

[Unreleased]: https://github.com/aki-akaguma/basexx/compare/v0.2.0..HEAD
[0.2.0]: https://github.com/aki-akaguma/basexx/compare/v0.1.1..v0.2.0
[0.1.1]: https://github.com/aki-akaguma/basexx/compare/v0.1.0..v0.1.1
[0.1.0]: https://github.com/aki-akaguma/basexx/releases/tag/v0.1.0
