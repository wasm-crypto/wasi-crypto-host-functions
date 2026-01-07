# WASI-Crypto host functions example implementation in Rust

This is a Rust implementation of the cryptographic extensions for WebAssembly.

It is meant to be compatible with the [WasmEdge implementation](https://wasmedge.org/docs/develop/rust/wasicrypto/).

These extensions significantly improve the efficiency of cryptographic operations in WebAssembly, while taking advantage of memory isolation, side channel protections and dedicated hardware support.

These APIs are stable, and have been designed to be simple and convenient to use in any programming language. They also minimize the risks of using cryptographic primitives insecurely.

The current implementations are focused on the most common use cases of cryptography in WebAssembly. They speed up encryption operations up to 100x, as well as authentication systems such as JWT and Passkeys. They provide an instant, net benefit to any WebAssembly application using cryptography.

New functionalities are planned, but we expect these to be incremental additions, that don't require any changes to applications using the current APIs.

## API documentation

The main documentation is here: [WASI Cryptography APIs](https://github.com/webassembly/wasi-crypto).

It includes the [specification](https://github.com/WebAssembly/wasi-crypto/blob/main/docs/wasi-crypto.md) as well as the entire set of types and function prototypes in various formats.

For automatic code and documentation generation, a description of these interfaces in the WITX 0.9 and WITX 0.10 formats are currently available. Translations to WAI and WIT will follow.

## The `wasi-crypto` crate

Code from this directory can also be found on `crates.io`: [`wasi-crypto`](https://crates.io/crates/wasi-crypto).

Using it as a dependency is recommended:

```toml
wasi-crypto = "0.1"
```

The crate documentation can be read online: [`wasi-crypto` documentation](https://docs.rs/wasi-crypto).

The `CryptoCtx` is where all the functions are defined. They closely match the specification, which acts as the documentation.

The `wasi-crypto` crate can be used in native applications, like a regular crypto library.

Here's a usage example:

```rust
use wasi_crypto::{CryptoCtx, CryptoError};
use ct_codecs::{Encoder, Hex};

fn main() -> Result<(), CryptoError> {
    let ctx = CryptoCtx::new();
    let h = ctx.symmetric_state_open("SHA-256", None, None)?;
    ctx.symmetric_state_absorb(h, b"Hello, world!")?;
    let mut digest = [0u8; 32];
    ctx.symmetric_state_squeeze(h, &mut digest)?;
    println!("SHA-256 digest: {}", Hex::encode_to_string(digest).unwrap());
    Ok(())
}
```

When using WebAssembly, the exact same functions, with the same interface, can be found in the following modules:

- `wasi_ephemeral_crypto_common`
- `wasi_ephemeral_crypto_asymmetric_common`
- `wasi_ephemeral_crypto_asymmetric_kx`
- `wasi_ephemeral_crypto_asymmetric_signatures`
- `wasi_ephemeral_crypto_asymmetric_symmetric`

Example abstraction layers, including for Rust, can be found here: [language bindings for WASI-Crypto](https://github.com/wasm-crypto/wasi-crypto-bindings).

## Usage with Wasmtime

The `wasi-crypto` crate exposes a standard Rust API in the `CryptoCtx` namespace.

But WebAssembly runtimes require different APIs.

This is why the crate also implements glue layers ("interfaces") between the Rust API and other APIs.

The first glue layer that has been implemented is for the Wasmtime runtime. In order to compile it,
add the `wasmtime` cargo feature when compiling the crate:

```toml
wasi-crypto = { version = "0.1", features = ["wasmtime"] }
```

When using `wasmtime` in a project written in Rust, the crypto extensions can be enabled in a
way that is similar to the core WASI functions, with the `add_to_linker()` function.

Example code to use `wasmtime` in Rust, with WASI and WASI-Crypto can be found here:
[wasmtime crate usage example](https://github.com/wasm-crypto/wasmtime-crypto/tree/wasi-crypto/wasmtime-crate-usage-example).

Since Wasmtime doesn't support plugins yet, new host functions cannot be added to the command-line tool nor the `libwasmtime` library, which is used by projects not written in Rust.

For conveniency, [`wasmtime-crypto`](https://github.com/wasm-crypto/wasmtime-crypto) is a stable version of `wasmtime` with support for `wasi-crypto`.

This includes the `wasmtime` command, as well as the `libwasmtime` library.

A different glue layer is required for every major version of `wasmtime` (the `wiggle` crate is not maintained independently), and having multiple versions of the same library doesn't work well in Rust. Which is why we only provide code for stable versions of `wasmtime`, as they are published to `crates.io`.
