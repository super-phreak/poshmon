use std::{error::Error, fmt};

use base64::encode;
use serde::{ser::{Serializer, SerializeMap}, Serialize, Deserialize};
use sha2::Sha256;
use hmac::{Hmac, Mac};
use tungstenite::Message;

use crate::{key::SessionToken, datagram::Datagram};

//use super::{structs::{Communication, Response, Commands}, keys::SessionToken};

const CURRENT_ALGO: &'static str = "HS256";
const PACKET_TYPE: &'static str = "PMT";
const VERSION: &'static str = "0.0.1";

pub type HmacSha256 = Hmac<Sha256>;

pub trait Communication {
    fn to_json_str(&self) -> String; 
    fn to_message(&self) -> Message; 
}

#[derive(Serialize)]
pub struct Packet {
    header: Header,
    data: Datagram,
    signature: String,
}

impl Packet {
    fn derive_signature(header: &Header, data: &String, session_token: &SessionToken) -> String {
        //Construct the header string.
        let header_b64 = encode(header.stringify());

        //For serialize the Packet should always be a response as it is an outgoing message.
        let body_b64 = encode(data);
        let msg = format!("{}.{}",header_b64,body_b64);

        //Sign the message as two b64 strings concatenated by a period.
        let mut mac = HmacSha256::new(&session_token.session_key);
        Mac::update(&mut mac, msg.as_bytes());
        let signature = mac.finalize();
        
        //Encode the signature into b64
        encode(signature.into_bytes().to_vec())

    }

    pub fn new(session_token: SessionToken, data: Datagram) -> Self {
        let header = Header::new();
        let data_json = data.to_json_str();
        let signature = Self::derive_signature(&header, &data_json, &session_token);

        Packet { header, data, signature }
    }

    pub fn verify(&self, token: SessionToken) -> Result<(), InvalidPacketError> {
        let header_b64 = encode(Header::new().stringify());
        let body_b64 = encode(&self.data.to_json_str());

        let msg = format!("{}.{}",header_b64,body_b64);
        let mut mac = HmacSha256::new(&token.session_key);
        println!("{}", &msg);
        mac.update(msg.as_bytes());
        let signature = mac.finalize();

        let bytes: Vec<u8> = signature.into_bytes().to_vec();
        let b64bytes = encode(bytes);
        println!("{}",b64bytes);

        if &self.signature == &b64bytes {
            return Ok(());
        }
        
        return Err(InvalidPacketError)
    }
}

impl Communication for Packet {
    fn to_json_str(&self) -> String {
        match serde_json::to_string(&self) {
            Ok(resp) => return resp,
            Err(_) => return String::from(format!("{{\"header\": \"{}\",\n \"data\": \"{}\",\n\"signature\": \"{}\"}}",self.header.stringify(),"ERR",self.signature))
        }
    }

    fn to_message(&self) -> Message {
        todo!()
    }
}

impl Communication for Datagram {
    fn to_json_str(&self) -> String {
        match serde_json::to_string(&self) {
            Ok(resp) => return resp,
            Err(_) => return String::from(format!("{{\"header\": \"{}\",\n \"data\": \"{}\",\n\"signature\": \"{}\"}}","ERR","ERR","ERR"))
        }
    }

    fn to_message(&self) -> Message {
        todo!()
    }
}

// impl serde::ser::Serialize for Packet {
    
//     fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
//     where
//         S: Serializer,
//     {
//         let mut s = serializer.serialize_map(None)?;
//         s.serialize_entry("header", &self.header)?;
//         s.serialize_entry("data", &self.data)?;
//         s.serialize_entry("signature", &self.signature)?;
//         s.end()
//     }
// }


#[derive(Serialize, Deserialize, Debug)]
struct Header {
    alg: String,
    typ: String,
    ver: String,
}

impl Header {
    fn new() -> Self{
        Header { alg: CURRENT_ALGO.to_owned(), typ: PACKET_TYPE.to_owned(), ver: VERSION.to_owned() }
    }

    //Use the JSON lib to create the string. If that fails build the string Manually
    fn stringify(&self) -> String {
        match serde_json::to_string(&self) {
            Ok(resp) => return resp,
            Err(_) => return String::from(format!("{{\"alg\": \"{}\",\n \"typ\": \"{}\",\n\"ver\": \"{}\"}}",CURRENT_ALGO,PACKET_TYPE,VERSION))
        }
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