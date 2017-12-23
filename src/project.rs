use uuid::Uuid;
#[cfg(feature = "wasm")]
use stdweb::unstable::TryFrom;
use rmps::to_vec;

pub const PROJECT_REQUEST_IDENTIFIER: u8 = 0;
#[derive(Debug, PartialEq, Deserialize, Serialize)]
pub struct ProjectRequest {
    pub id: Uuid
}

impl ProjectRequest {
    pub fn new(id: Uuid) -> Vec<u8> {
        let req = ProjectRequest { id };
        let mut vec = to_vec(&req).unwrap();
        vec.push(PROJECT_REQUEST_IDENTIFIER);
        vec
    }
}

pub const PROJECT_IDENTIFIER: u8 = 1;
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
}

#[cfg(feature = "wasm")]
js_serializable!( Project );
#[cfg(feature = "wasm")]
js_deserializable!( Project );
