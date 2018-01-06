use uuid::Uuid;
#[cfg(feature = "wasm")]
use stdweb::unstable::TryFrom;

use identifier;
use serialize::serialize;

#[derive(Debug, PartialEq, Deserialize, Serialize)]
pub struct ProjectRequest {
    pub id: Uuid,
    pub track_file_tree: bool,
}

impl ProjectRequest {
    pub fn new(id: Uuid, track_file_tree: bool) -> ProjectRequest {
        ProjectRequest {
            id,
            track_file_tree,
        }
    }

    pub fn serialize(&self) -> Vec<u8> {
        serialize(self, identifier::PROJECT_REQUEST)
    }
}

#[derive(Debug, PartialEq, Deserialize, Serialize)]
pub struct ProjectRequestError {
    pub reason: String,
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
js_serializable!(ProjectRequestError);
#[cfg(feature = "wasm")]
js_deserializable!(ProjectRequestError);

#[derive(Debug, PartialEq, Deserialize, Serialize)]
pub struct Project {
    id: Uuid,
    name: String,
    connected_sessions: Vec<Uuid>,
}

impl Project {
    pub fn new(id: Uuid, name: String, connected_sessions: Vec<Uuid>) -> Project {
        Project {
            id,
            name,
            connected_sessions,
        }
    }

    pub fn serialize(&self) -> Vec<u8> {
        serialize(self, identifier::PROJECT)
    }
}

#[cfg(feature = "wasm")]
js_serializable!(Project);
#[cfg(feature = "wasm")]
js_deserializable!(Project);

#[derive(Debug, PartialEq, Deserialize, Serialize)]
pub struct FileTreeNode {
    nodes: Vec<FileTreeNode>,
    content: Option<Uuid>,
}

impl FileTreeNode {
    fn new() -> FileTreeNode {
        FileTreeNode {
            nodes: Vec::new(),
            content: None,
        }
    }
}

#[cfg(feature = "wasm")]
js_serializable!(FileTreeNode);
#[cfg(feature = "wasm")]
js_deserializable!(FileTreeNode);

#[derive(Debug, PartialEq, Deserialize, Serialize)]
pub struct FileTree {
    root: FileTreeNode,
}

impl FileTree {
    pub fn new() -> FileTree {
        FileTree {
            root: FileTreeNode::new(),
        }
    }

    pub fn serialize(&self) -> Vec<u8> {
        serialize(self, identifier::FILE_TREE)
    }
}

#[cfg(feature = "wasm")]
js_serializable!(FileTree);
#[cfg(feature = "wasm")]
js_deserializable!(FileTree);
