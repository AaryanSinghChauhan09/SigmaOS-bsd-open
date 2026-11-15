# SigmaOS-bsd-open

> **OpenBSD** — BSD system persona for SigmaOS.

Part of the [SigmaOS](https://github.com/AaryanSinghChauhan09/SigmaOS) ecosystem.  
[pledge(), unveil(), W^X, pf firewall](https://openbsd.org)

## Quickstart

```bash
cargo test    # run deterministic AAA unit tests
cargo check   # verify compilation
```

## Component Role

This crate implements the `open` subsystem for SigmaOS:
- Fully real Rust implementation (no stubs)
- Fail-fast validation with typed errors
- Deterministic unit tests (Arrange-Act-Assert pattern)
- Single Responsibility, Modular, Zero-Copy where applicable

## API

```rust
use sigmaos_bsd_open::*;
// See src/lib.rs for full public API
```

## Documentation

Full documentation is in the [SigmaOS GitHub Wiki → BSD-Personas](https://github.com/AaryanSinghChauhan09/SigmaOS/wiki/BSD-Personas).

## License

BSD-2-Clause
