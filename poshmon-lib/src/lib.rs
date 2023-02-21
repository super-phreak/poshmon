pub mod packet;
pub mod datagram;
pub mod key;

pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use crate::{datagram::Datagram, packet::Communication};

    use super::*;

    

    #[test]
    fn packet_test() {
        let token = key::SessionToken::new("testuser".to_owned());
        let team = Datagram::Awk {session_id: token.session_id.to_string(), cmd_response: "Failure to submit team".to_string()};
        let packet = packet::Packet::new(token.clone(), team);
        println!("{}", packet.to_json_str());
        assert!(packet.verify(token).is_ok())

    }
}
