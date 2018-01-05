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
pub struct Project {
    pub id: Uuid,
    pub name: String
}

impl Project {
    pub fn mock() -> Project {
        Project {
            id: Uuid::parse_str("936DA01F9ABD4d9d80C702AF85C822A8").unwrap(),
            name: "SomeName".to_string()
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
