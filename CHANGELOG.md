# Changelog: basexx

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]
### Added
- Introduce `bytemuck` crate for safer memory casting and alignment handling.

### Fixed
- Resolve potential Undefined Behavior (UB) in SIMD implementations by replacing unaligned pointer dereferences with safe `bytemuck` primitives and `std::ptr::copy_nonoverlapping`.
- Fix broken benchmark build caused by missing legacy `aligned_data` module.

### Changed
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

[Unreleased]: https://github.com/aki-akaguma/basexx/compare/v0.1.1..HEAD
[0.1.1]: https://github.com/aki-akaguma/basexx/compare/v0.1.0..v0.1.1
[0.1.0]: https://github.com/aki-akaguma/basexx/releases/tag/v0.1.0
