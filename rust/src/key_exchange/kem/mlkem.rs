use ml_kem::kem::{Decapsulate, Encapsulate, Kem, KeyExport, KeyInit, TryKeyInit};
use ml_kem::{DecapsulationKey, EncapsulationKey, MlKem512, MlKem768, MlKem1024};

use super::*;

macro_rules! mlkem_dispatch {
    ($alg:expr, $p:ident => $body:block) => {
        match $alg {
            KxAlgorithm::MlKem512 => {
                type $p = MlKem512;
                $body
            }
            KxAlgorithm::MlKem768 => {
                type $p = MlKem768;
                $body
            }
            KxAlgorithm::MlKem1024 => {
                type $p = MlKem1024;
                $body
            }
            _ => bail!(CryptoError::UnsupportedAlgorithm),
        }
    };
}

fn generate(alg: KxAlgorithm) -> Result<(Vec<u8>, Vec<u8>), CryptoError> {
    Ok(mlkem_dispatch!(alg, P => {
        let (dk, ek) = P::generate_keypair();
        (ek.to_bytes().to_vec(), dk.to_bytes().to_vec())
    }))
}

fn encapsulate(alg: KxAlgorithm, pk_raw: &[u8]) -> Result<EncapsulatedSecret, CryptoError> {
    let (secret, encapsulated_secret) = mlkem_dispatch!(alg, P => {
        let ek = EncapsulationKey::<P>::new_from_slice(pk_raw).map_err(|_| CryptoError::InvalidKey)?;
        let (ciphertext, secret) = ek.encapsulate();
        (secret.to_vec(), ciphertext.to_vec())
    });
    Ok(EncapsulatedSecret {
        secret,
        encapsulated_secret,
    })
}

fn decapsulate(alg: KxAlgorithm, sk_raw: &[u8], encapsulated_secret: &[u8]) -> Result<Vec<u8>, CryptoError> {
    Ok(mlkem_dispatch!(alg, P => {
        let dk = DecapsulationKey::<P>::new_from_slice(sk_raw).map_err(|_| CryptoError::InvalidKey)?;
        dk.decapsulate_slice(encapsulated_secret)
            .map_err(|_| CryptoError::VerificationFailed)?
            .to_vec()
    }))
}

fn derive_publickey(alg: KxAlgorithm, sk_raw: &[u8]) -> Result<Vec<u8>, CryptoError> {
    Ok(mlkem_dispatch!(alg, P => {
        let dk = DecapsulationKey::<P>::new_from_slice(sk_raw).map_err(|_| CryptoError::InvalidKey)?;
        dk.encapsulation_key().to_bytes().to_vec()
    }))
}

#[derive(Clone, Debug)]
pub struct MlKemPublicKey {
    alg: KxAlgorithm,
    raw: Vec<u8>,
}

#[derive(Clone, Derivative)]
#[derivative(Debug)]
pub struct MlKemSecretKey {
    alg: KxAlgorithm,
    #[derivative(Debug = "ignore")]
    raw: Vec<u8>,
}

#[derive(Clone, Debug)]
pub struct MlKemKeyPair {
    alg: KxAlgorithm,
    pk: MlKemPublicKey,
    sk: MlKemSecretKey,
}

pub struct MlKemKeyPairBuilder {
    alg: KxAlgorithm,
}

impl MlKemKeyPairBuilder {
    pub fn new(alg: KxAlgorithm) -> Box<dyn KxKeyPairBuilder> {
        Box::new(Self { alg })
    }
}

impl KxKeyPairBuilder for MlKemKeyPairBuilder {
    fn generate(&self, _options: Option<KxOptions>) -> Result<KxKeyPair, CryptoError> {
        let (pk_raw, sk_raw) = generate(self.alg)?;
        let pk = MlKemPublicKey {
            alg: self.alg,
            raw: pk_raw,
        };
        let sk = MlKemSecretKey {
            alg: self.alg,
            raw: sk_raw,
        };
        let kp = MlKemKeyPair {
            alg: self.alg,
            pk,
            sk,
        };
        Ok(KxKeyPair::new(Box::new(kp)))
    }
}

impl KxKeyPairLike for MlKemKeyPair {
    fn alg(&self) -> KxAlgorithm {
        self.alg
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

impl KxPublicKeyLike for MlKemPublicKey {
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn alg(&self) -> KxAlgorithm {
        self.alg
    }

    fn len(&self) -> Result<usize, CryptoError> {
        Ok(self.raw.len())
    }

    fn as_raw(&self) -> Result<&[u8], CryptoError> {
        Ok(&self.raw)
    }

    fn encapsulate(&self) -> Result<EncapsulatedSecret, CryptoError> {
        encapsulate(self.alg, &self.raw)
    }
}

impl KxSecretKeyLike for MlKemSecretKey {
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn alg(&self) -> KxAlgorithm {
        self.alg
    }

    fn len(&self) -> Result<usize, CryptoError> {
        Ok(self.raw.len())
    }

    fn as_raw(&self) -> Result<&[u8], CryptoError> {
        Ok(&self.raw)
    }

    fn publickey(&self) -> Result<KxPublicKey, CryptoError> {
        let raw = derive_publickey(self.alg, &self.raw)?;
        Ok(KxPublicKey::new(Box::new(MlKemPublicKey {
            alg: self.alg,
            raw,
        })))
    }

    fn decapsulate(&self, encapsulated_secret: &[u8]) -> Result<Vec<u8>, CryptoError> {
        decapsulate(self.alg, &self.raw, encapsulated_secret)
    }
}
