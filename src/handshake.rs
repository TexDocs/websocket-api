use uuid::Uuid;
#[cfg(feature = "wasm")]
use stdweb::unstable::TryFrom;

use identifier;
use serialize::serialize;

#[derive(Debug, PartialEq, Deserialize, Serialize)]
pub struct Handshake {
    protocol_version: String
}

impl Handshake {
    pub fn new() -> Handshake {
        Handshake {
            protocol_version: String::from(identifier::PROTOCOL_VERSION)
        }
    }

    pub fn serialize(&self) -> Vec<u8> {
        serialize(self, identifier::HANDSHAKE)
    }
}

#[cfg(feature = "wasm")]
js_serializable!( Handshake );
#[cfg(feature = "wasm")]
js_deserializable!( Handshake );


#[derive(Debug, PartialEq, Deserialize, Serialize)]
pub struct HandshakeError {
    pub reason: String
}

impl HandshakeError {
    pub fn new(msg: String) -> HandshakeError {
        HandshakeError { reason: msg }
    }

    pub fn serialize(&self) -> Vec<u8> {
        serialize(self, identifier::HANDSHAKE_ERR)
    }
}

#[cfg(feature = "wasm")]
js_serializable!( HandshakeError );
#[cfg(feature = "wasm")]
js_deserializable!( HandshakeError );


#[derive(Debug, PartialEq, Deserialize, Serialize)]
pub struct HandshakeAcknowledgement {
    pub session_id: Uuid
}

impl HandshakeAcknowledgement {
    pub fn new(session_id: Uuid) -> HandshakeAcknowledgement {
        HandshakeAcknowledgement { session_id }
    }

    pub fn serialize(&self) -> Vec<u8> {
        serialize(self, identifier::HANDSHAKE_ACK)
    }
}

#[cfg(feature = "wasm")]
js_serializable!( HandshakeAcknowledgement );
#[cfg(feature = "wasm")]
js_deserializable!( HandshakeAcknowledgement );
