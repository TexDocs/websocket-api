#![feature(try_from)]
#![feature(underscore_lifetimes)]

extern crate rmp_serde as rmps;
extern crate serde;
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate stdweb;
extern crate uuid;

use stdweb::unstable::TryInto;
use stdweb::{Null, Value};
use uuid::Uuid;

// Deserialize to JS
mod identifier;
mod serialize;
use serialize::deserialize_to_js;

// Handshake
mod handshake;
use handshake::*;

// Project
mod project;
use project::*;

// User management
mod user;
use user::*;

fn create_handshake() -> Value {
    Handshake::new().serialize().try_into().unwrap()
}

fn request_project(id: Value, track_file_tree: Value) -> Value {
    let id: String = id.try_into().unwrap();
    let parsed_id = Uuid::parse_str(&id).unwrap();

    ProjectRequest::new(parsed_id, track_file_tree.try_into().unwrap())
        .serialize()
        .try_into()
        .unwrap()
}

fn parse_msg(msg: Value) -> Value {
    let mut data: Vec<u8> = msg.try_into().unwrap();
    let id = data.pop().unwrap();

    match id {
        // Handshake
        identifier::HANDSHAKE_ACK => deserialize_to_js::<HandshakeAcknowledgement>(
            &data,
            String::from("HandshakeAcknowledgement"),
        ),
        identifier::HANDSHAKE_ERR => {
            deserialize_to_js::<HandshakeError>(&data, String::from("HandshakeError"))
        }

        // Project
        identifier::PROJECT => deserialize_to_js::<Project>(&data, String::from("Project")),
        identifier::PROJECT_REQUEST_ERR => {
            deserialize_to_js::<ProjectRequestError>(&data, String::from("ProjectRequestError"))
        }
        identifier::FILE_TREE => deserialize_to_js::<FileTree>(&data, String::from("FileTree")),

        // User management
        identifier::USER_JOINED => {
            deserialize_to_js::<UserJoined>(&data, String::from("UserJoined"))
        }
        identifier::USER_LEFT => deserialize_to_js::<UserLeft>(&data, String::from("UserLeft")),

        // Else
        _ => Null.try_into().unwrap(),
    }
}

fn main() {
    stdweb::initialize();

    js! {
        Module.exports.createHandshake = @{create_handshake};
        Module.exports.requestProject = @{request_project};
        Module.exports.parseMessage = @{parse_msg};
    }
}
