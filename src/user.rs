use uuid::Uuid;
#[cfg(feature = "wasm")]
use stdweb::unstable::TryFrom;

use identifier;
use serialize::serialize;

#[derive(Debug, PartialEq, Deserialize, Serialize)]
pub struct UserJoined {
    session_id: Uuid,
}

impl UserJoined {
    pub fn new(session_id: Uuid) -> UserJoined {
        UserJoined { session_id }
    }

    pub fn serialize(&self) -> Vec<u8> {
        serialize(self, identifier::USER_JOINED)
    }
}

#[cfg(feature = "wasm")]
js_serializable!(UserJoined);
#[cfg(feature = "wasm")]
js_deserializable!(UserJoined);

#[derive(Debug, PartialEq, Deserialize, Serialize)]
pub struct UserLeft {
    session_id: Uuid,
}

impl UserLeft {
    pub fn new(session_id: Uuid) -> UserLeft {
        UserLeft { session_id }
    }

    pub fn serialize(&self) -> Vec<u8> {
        serialize(self, identifier::USER_LEFT)
    }
}

#[cfg(feature = "wasm")]
js_serializable!(UserLeft);
#[cfg(feature = "wasm")]
js_deserializable!(UserLeft);
