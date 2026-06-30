#[cfg(feature = "pqcrypto")]
mod mlkem;
#[cfg(feature = "pqcrypto")]
mod xwing;

#[cfg(feature = "pqcrypto")]
pub use mlkem::*;
#[cfg(feature = "pqcrypto")]
pub use xwing::*;

use super::*;

#[derive(Clone, Debug)]
pub struct EncapsulatedSecret {
    pub encapsulated_secret: Vec<u8>,
    pub secret: Vec<u8>,
}
