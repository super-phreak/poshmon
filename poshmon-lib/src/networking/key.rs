use crypto_common::{rand_core::{CryptoRng, RngCore, OsRng}, Key, KeyInit, KeySizeUser};
use hmac::Hmac;
use sha2::Sha256;
use core::fmt::Debug;
use std::fmt::Display;
use digest::consts::U16;
use serde::ser::{Serialize, Serializer, SerializeStruct};
use uuid::Uuid;
use base64::prelude::*;

pub type HmacSha256 = Hmac<Sha256>;

pub struct Salt;

impl KeySizeUser for Salt {
    type KeySize = U16;     
}

impl KeyInit for Salt {
    fn new(_key: &Key<Self>) -> Self {
        todo!()
    }

    fn new_from_slice(key: &[u8]) -> Result<Self, crypto_common::InvalidLength> {
        if key.len() != Self::key_size() {
            Err(crypto_common::InvalidLength)
        } else {
            Ok(Self::new(Key::<Self>::from_slice(key)))
        }
    }

    fn generate_key(mut rng: impl CryptoRng + RngCore) -> Key<Self> { 
        let mut key = Key::<Self>::default();
        rng.fill_bytes(&mut key);
        key
    }


}

#[derive(Clone)]
pub struct SessionToken {
    pub session_id: Uuid,
    pub username: String,
    pub session_key: Key<HmacSha256>, 
}

impl SessionToken {

    pub fn new(username: String) -> SessionToken {
        println!("New Session generated: {}", username);
        let rng = OsRng::default();
        let session_key: Key<HmacSha256> =  HmacSha256::generate_key(rng);
        SessionToken {
            session_id: Uuid::new_v4(),
            username,
            session_key,
        }
    }

    pub fn key_as_string(&self) -> String {
        BASE64_STANDARD.encode(&self.session_key)
    }
}

impl Serialize for SessionToken {
    
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut s = serializer.serialize_struct("SessionToken", 2)?;
        s.serialize_field("session_id", &self.session_id.to_string())?;
        s.serialize_field("username", &self.username)?;
        s.end()
    }
}

impl Debug for SessionToken {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("SessionToken").field("session_id", &self.session_id.to_string()).field("username", &self.username).field("session_key", &BASE64_STANDARD.encode(&self.session_key)).finish()
    }
}

impl Display for SessionToken {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "session id: {}, username: {}", &self.session_id.to_string(), &self.username)
    }
}