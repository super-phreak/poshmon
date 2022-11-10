use std::{error::Error, fmt};

use base64::encode;
use serde::{ser::{Serializer, SerializeMap, SerializeStruct}, Serialize, Deserialize};
use sha2::{Sha256, Sha512};
use hmac::{Hmac, Mac};

use super::{structs::{Communication, Response, Commands}, keys::SessionToken};

const CURRENT_ALGO: &'static str = "HS256";
const PACKET_TYPE: &str = "PMT";
const VERSION: &str = "0.0.1";

pub type HmacSha256 = Hmac<Sha256>;

pub struct OutPacket {
    session_token: SessionToken,
    data: Response,
}

impl OutPacket {
    pub fn new(session_token: SessionToken, data: Response) -> Self {
        OutPacket {session_token, data}
    }
}

#[derive(Deserialize, Debug)]
pub struct InPacket {
    header: Header,
    body: Commands,
    signature: String,
}

impl Communication for OutPacket {}
impl Communication for Header {}

#[derive(Serialize, Deserialize, Debug)]
struct Header {
    alg: String,
    typ: String,
    ver: String,
    session_id: String,
}

impl serde::ser::Serialize for OutPacket {
    
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        //Construct the header string.
        //let header_string = format!("{{\"alg\": \"{}\",\n \"typ\": \"{}\",\n\"ver\": \"{}\",\n \"session_id\": \"{}\"}}",CURRENT_ALGO,PACKET_TYPE,VERSION,&self.session_token.session_id.to_string());
        let header: Header = Header {alg: CURRENT_ALGO.to_string(), typ: PACKET_TYPE.to_string(), ver: VERSION.to_string(), session_id: self.session_token.session_id.to_string().clone() };
        let header_b64 = encode(&header.to_json_str());

        //For serialize the Packet should always be a response as it is an outgoing message.
        let body_b64 = encode(&self.data.to_json_str());
        let msg = format!("{}.{}",header_b64,body_b64);

        //Sign the message as two b64 strings concatenated by a period.
        let mut mac = HmacSha256::new(&self.session_token.session_key);
        println!("{}", &msg);
        Mac::update(&mut mac, msg.as_bytes());
        let signature = mac.finalize();

        //Encode the signature into b64
        let bytes: Vec<u8> = signature.into_bytes().to_vec();
        let b64bytes = encode(bytes);

        let mut s = serializer.serialize_map(None)?;
        s.serialize_entry("header", &header)?;
        s.serialize_entry("body", &self.data)?;
        s.serialize_entry("signature", &b64bytes)?;
        s.end()
    }
}

impl InPacket {
    fn verify(&self, token: SessionToken) -> Result<(), InvalidPacketError> {
        let header_b64 = encode(&self.header.to_json_str());
        let body_b64 = encode(&self.body.to_json_str());

        let msg = format!("{}.{}",header_b64,body_b64);
        let mut mac = HmacSha256::new(&token.session_key);
        println!("{}", &msg);
        mac.update(msg.as_bytes());
        let signature = mac.finalize();

        let bytes: Vec<u8> = signature.into_bytes().to_vec();
        let b64bytes = encode(bytes);

        if &self.signature == &b64bytes {
            return Ok(());
        }
        
        return Err(InvalidPacketError)
    }
}

#[derive(Debug)]
pub struct InvalidPacketError;
impl fmt::Display for InvalidPacketError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Packet was invalid")
    }
}

impl Error for InvalidPacketError {}