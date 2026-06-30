use x_wing::{
    Decapsulate, Decapsulator, DecapsulationKey, Encapsulate, EncapsulationKey, Kem, KeyExport,
    KeyInit, TryKeyInit, XWingKem,
};

use super::*;

fn generate() -> (Vec<u8>, Vec<u8>) {
    let (dk, ek) = XWingKem::generate_keypair();
    (ek.to_bytes().to_vec(), dk.as_bytes().to_vec())
}

fn encapsulate(pk_raw: &[u8]) -> Result<EncapsulatedSecret, CryptoError> {
    let ek = EncapsulationKey::new_from_slice(pk_raw).map_err(|_| CryptoError::InvalidKey)?;
    let (ciphertext, secret) = ek.encapsulate();
    Ok(EncapsulatedSecret {
        secret: secret.to_vec(),
        encapsulated_secret: ciphertext.to_vec(),
    })
}

fn decapsulate(sk_raw: &[u8], encapsulated_secret: &[u8]) -> Result<Vec<u8>, CryptoError> {
    let dk = DecapsulationKey::new_from_slice(sk_raw).map_err(|_| CryptoError::InvalidKey)?;
    Ok(dk
        .decapsulate_slice(encapsulated_secret)
        .map_err(|_| CryptoError::VerificationFailed)?
        .to_vec())
}

fn derive_publickey(sk_raw: &[u8]) -> Result<Vec<u8>, CryptoError> {
    let dk = DecapsulationKey::new_from_slice(sk_raw).map_err(|_| CryptoError::InvalidKey)?;
    Ok(dk.encapsulation_key().to_bytes().to_vec())
}

#[derive(Clone, Debug)]
pub struct XWingPublicKey {
    raw: Vec<u8>,
}

#[derive(Clone, Derivative)]
#[derivative(Debug)]
pub struct XWingSecretKey {
    #[derivative(Debug = "ignore")]
    raw: Vec<u8>,
}

#[derive(Clone, Debug)]
pub struct XWingKeyPair {
    pk: XWingPublicKey,
    sk: XWingSecretKey,
}

pub struct XWingKeyPairBuilder;

impl XWingKeyPairBuilder {
    pub fn new(_alg: KxAlgorithm) -> Box<dyn KxKeyPairBuilder> {
        Box::new(Self)
    }
}

impl KxKeyPairBuilder for XWingKeyPairBuilder {
    fn generate(&self, _options: Option<KxOptions>) -> Result<KxKeyPair, CryptoError> {
        let (pk_raw, sk_raw) = generate();
        let kp = XWingKeyPair {
            pk: XWingPublicKey { raw: pk_raw },
            sk: XWingSecretKey { raw: sk_raw },
        };
        Ok(KxKeyPair::new(Box::new(kp)))
    }
}

impl KxKeyPairLike for XWingKeyPair {
    fn alg(&self) -> KxAlgorithm {
        KxAlgorithm::XWing
    }

    fn as_any(&self) -> &dyn Any {
        self
    }

    fn publickey(&self) -> Result<KxPublicKey, CryptoError> {
        Ok(KxPublicKey::new(Box::new(self.pk.clone())))
    }

    fn secretkey(&self) -> Result<KxSecretKey, CryptoError> {
        Ok(KxSecretKey::new(Box::new(self.sk.clone())))
    }
}

impl KxPublicKeyLike for XWingPublicKey {
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn alg(&self) -> KxAlgorithm {
        KxAlgorithm::XWing
    }

    fn len(&self) -> Result<usize, CryptoError> {
        Ok(self.raw.len())
    }

    fn as_raw(&self) -> Result<&[u8], CryptoError> {
        Ok(&self.raw)
    }

    fn encapsulate(&self) -> Result<EncapsulatedSecret, CryptoError> {
        encapsulate(&self.raw)
    }
}

impl KxSecretKeyLike for XWingSecretKey {
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn alg(&self) -> KxAlgorithm {
        KxAlgorithm::XWing
    }

    fn len(&self) -> Result<usize, CryptoError> {
        Ok(self.raw.len())
    }

    fn as_raw(&self) -> Result<&[u8], CryptoError> {
        Ok(&self.raw)
    }

    fn publickey(&self) -> Result<KxPublicKey, CryptoError> {
        let raw = derive_publickey(&self.raw)?;
        Ok(KxPublicKey::new(Box::new(XWingPublicKey { raw })))
    }

    fn decapsulate(&self, encapsulated_secret: &[u8]) -> Result<Vec<u8>, CryptoError> {
        decapsulate(&self.raw, encapsulated_secret)
    }
}
