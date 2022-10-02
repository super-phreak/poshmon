use base64::encode;
use serde::{ser::{Serializer, SerializeMap}, Serialize, Deserialize};
use sha2::Sha256;
use hmac::{Hmac, Mac};

use super::{auth::SessionToken, structs::{Communication, Response}};

const CURRENT_ALGO: &str = "HS256";
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

// #[derive(Deserialize, Debug)]
// pub struct InPacket {
//     header: Header,
//     body: Commands,
//     signature: String,
// }

impl Communication for OutPacket {}

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
        let header_string = format!("{{\"alg\": \"{}\",\n \"typ\": \"{}\",\n\"ver\": \"{}\",\n \"session_id\": \"{}\"}}",CURRENT_ALGO,PACKET_TYPE,VERSION,&self.session_token.session_id.to_string());
        let header_b64 = encode(&header_string);
        let header: Header = serde_json::from_str(&header_string).unwrap();

        //For serialize the Packet should always be a response as it is an outgoing message.
        let body_b64 = encode(&self.data.to_json_str());
        let msg = format!("{}.{}",header_b64,body_b64);

        //Sign the message as two b64 strings concatenated by a period.
        let mut mac = HmacSha256::new(&self.session_token.session_key);
        mac.update(msg.as_bytes());
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

