use std::error::Error;
use crypto_common::{Key, KeyInit};
use hmac::HmacCore;
use serde::ser::{Serialize, Serializer, SerializeStruct};
use sha2::Sha256;
use uuid::Uuid;
use crypto_common::rand_core::{OsRng};

use super::crypto::HmacSha256;

#[derive(Debug)]
pub struct SessionToken {
    pub session_id: Uuid,
    pub username: String,
    pub session_key: Key<HmacSha256>, 
}

impl SessionToken {

    fn new(username: String) -> SessionToken {
        let rng = OsRng::default();
        let session_key: Key<HmacCore<Sha256>> =  HmacSha256::generate_key(rng);
        SessionToken {
            session_id: Uuid::new_v4(),
            username,
            session_key,
        }
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

pub fn login(username: String, password: String) -> Result<SessionToken, Box<dyn Error>> {
    _ = password;
    Ok(SessionToken::new(username))
}