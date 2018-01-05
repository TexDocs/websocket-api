use uuid::Uuid;
#[cfg(feature = "wasm")]
use stdweb::unstable::TryFrom;

use identifier;
use serialize::serialize;

#[derive(Debug, PartialEq, Deserialize, Serialize)]
pub struct ProjectRequest {
    pub id: Uuid
}

impl ProjectRequest {
    pub fn new(id: Uuid) -> ProjectRequest {
        ProjectRequest { id }
    }

    pub fn serialize(&self) -> Vec<u8> {
        serialize(self, identifier::PROJECT_REQUEST)
    }
}

#[derive(Debug, PartialEq, Deserialize, Serialize)]
pub struct ProjectRequestError {
    pub reason: String
}

impl ProjectRequestError {
    pub fn new(msg: String) -> ProjectRequestError {
        ProjectRequestError { reason: msg }
    }

    pub fn serialize(&self) -> Vec<u8> {
        serialize(self, identifier::PROJECT_REQUEST_ERR)
    }
}

#[cfg(feature = "wasm")]
js_serializable!( ProjectRequestError );
#[cfg(feature = "wasm")]
js_deserializable!( ProjectRequestError );

#[derive(Debug, PartialEq, Deserialize, Serialize)]
pub struct Project {
    pub id: Uuid,
    pub name: String,
    pub x: u8,
}

impl Project {
    pub fn new(id: Uuid, name: String ) -> Project {
        Project { id, name, x: 255 }
    }

    pub fn mock() -> Project {
        Project {
            id: Uuid::parse_str("936DA01F9ABD4d9d80C702AF85C822A8").unwrap(),
            name: "SomeName".to_string(),
            x: 255
        }
    }

    pub fn serialize(&self) -> Vec<u8> {
        serialize(self, identifier::PROJECT)
    }
}

#[cfg(feature = "wasm")]
js_serializable!( Project );
#[cfg(feature = "wasm")]
js_deserializable!( Project );
