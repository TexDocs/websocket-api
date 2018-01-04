#![feature(try_from)]
#![feature(underscore_lifetimes)]

#[macro_use]
extern crate stdweb;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate rmp_serde as rmps;
extern crate uuid;

mod serialize;
mod identifier;
mod project;
mod handshake;

use stdweb::unstable::TryInto;
use stdweb::{Value, Null};
use project::*;
use handshake::*;
use serialize::deserialize_to_js;
use uuid::Uuid;

fn create_handshake() -> Value {
    Handshake::new().serialize().try_into().unwrap()
}

fn request_project(id: Value) -> Value {
    let id: String = id.try_into().unwrap();
    let parsed_id = Uuid::parse_str(&id).unwrap();

    ProjectRequest::new(parsed_id).serialize().try_into().unwrap()
}

fn parse_msg(msg: Value) -> Value {
    let mut data: Vec<u8> = msg.try_into().unwrap();
    let id = data.pop().unwrap();

    match id {
        identifier::PROJECT => deserialize_to_js::<Project>(&data),
        identifier::HANDSHAKE_ACK => deserialize_to_js::<HandshakeAcknowledgement>(&data),
        identifier::HANDSHAKE_ERR => deserialize_to_js::<HandshakeError>(&data),
        _ => Null.try_into().unwrap()
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
