use std::collections::HashMap;

pub use super::CryptoCtx as WasiCryptoCtx;

pub fn witx_interfaces() -> HashMap<&'static str, &'static str> {
    let mut map = HashMap::new();
    map.insert(
        "proposal_common.witx",
        include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/api/witx-0.9/proposal_common.witx"
        )),
    );
    map.insert(
        "proposal_asymmetric_common.witx",
        include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/api/witx-0.9/proposal_asymmetric_common.witx"
        )),
    );
    map.insert(
        "proposal_signatures.witx",
        include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/api/witx-0.9/proposal_signatures.witx"
        )),
    );
    map.insert(
        "proposal_symmetric.witx",
        include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/api/witx-0.9/proposal_symmetric.witx"
        )),
    );
    map.insert(
        "proposal_external_secrets.witx",
        include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/api/witx-0.9/proposal_external_secrets.witx"
        )),
    );
    map.insert(
        "proposal_kx.witx",
        include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/api/witx-0.9/proposal_kx.witx"
        )),
    );
    map.insert(
        "wasi_ephemeral_crypto.witx",
        include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/api/witx-0.9/wasi_ephemeral_crypto.witx"
        )),
    );
    map
}

wiggle::from_witx!({
    witx: ["$CARGO_MANIFEST_DIR/api/witx-0.9/wasi_ephemeral_crypto.witx"],
});

pub mod wasi_modules {
    pub use super::{
        wasi_ephemeral_crypto_asymmetric_common, wasi_ephemeral_crypto_common,
        wasi_ephemeral_crypto_kx, wasi_ephemeral_crypto_signatures,
        wasi_ephemeral_crypto_symmetric,
    };
}

pub use types as guest_types;

mod asymmetric_common;
mod common;
mod error;
mod key_exchange;
mod signatures;
mod symmetric;
